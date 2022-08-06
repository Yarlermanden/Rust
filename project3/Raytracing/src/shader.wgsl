struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
    inv_view_mat: mat4x4<f32>,
    inv_proj_mat: mat4x4<f32>,
}
@group(0) @binding(0)
var<uniform> camera: Camera;

struct Model {
    current_time: f32,
    padding: f32,
    padding2: f32,
    padding3: f32,
}
@group(1) @binding(0)
var<uniform> model: Model;

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) view_pos: vec3<f32>,
    @location(1) color: vec3<f32>,
};


@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    var viewPos = camera.inv_proj_mat * out.clip_position;
    out.view_pos = viewPos.xyz / viewPos.w;
    out.color = model.position;
    return out;
}




let _rm_MaxRays: i32 = 2;
fn getInfinity() -> f32 { return 1.0 / 0.0; }

struct Ray
{
    location: vec3<f32>,
    direction: vec3<f32>,
    colorFilter: vec3<f32>,
};

//var _rt_pendingRays: array<Ray, 100>;
//var _rt_rayCount: i32;

struct Material
{
    color: vec3<f32>,
};

struct Output
{
    location: vec3<f32>,
    normal: vec3<f32>,
    refractPoint: vec3<f32>,
    refractDirection: vec3<f32>,
    material: Material,
};

struct Sphere
{
    center: vec3<f32>,
    radius: f32,
    material: Material,
};

struct Distance 
{
    d: f32,
};

struct Globals {
    rayCount: i32,
    pendingRays: array<Ray, _rm_MaxRays>,
};

fn raySphereIntersection(ray: Ray, sphere: Sphere, distance: ptr<function, f32>, o: ptr<function, Output>) -> bool
{
    var hit = false;

    let m: vec3<f32> = ray.location - sphere.center;
    let b = dot(m, ray.direction);
    let c = dot(m, m) - sphere.radius * sphere.radius;

    if (c <= 0.0 || b <= 0.0)
    {
        let discr = b * b - c;
        if (discr >= 0.0)
        {
            let d = max(-b - sqrt(discr), 0.0);
            if (d < *distance)
            {
                *distance = d;
                (*o).location = ray.location + d * ray.direction;
                (*o).normal = normalize((*o).location - sphere.center);
                (*o).material = sphere.material;
                hit = true;
            }
        }
    }
    return hit;
}

//output o....
fn castRay(ray: Ray, distance: ptr<function, f32>, o: ptr<function, Output>) -> bool
{
    var sphere: Sphere;
    sphere.center = vec3<f32>(0.0, 0.0, -10.0);
    sphere.radius = 2.0;
    sphere.material.color = vec3<f32>(1.0);
    let _rt_Time = model.current_time;

    var hit = false;
    for (var i = 1; i <= 10; i+=1)
    {
        let i2 = f32(i);
        let offset = 5.0 * vec3<f32>(sin(3.0*i2+_rt_Time), sin(2.0*i2+_rt_Time), sin(4.0*i2+_rt_Time));
        sphere.center = offset + vec3<f32>(0.0, 0.0, -20.0);
        sphere.material.color = normalize(offset) * 0.5 + 0.5;
        hit = raySphereIntersection(ray, sphere, distance, o) || hit;
    }
    return hit;
}

fn castRay1(ray: Ray, distance: ptr<function, f32>) -> bool
{
    var output: Output;
    //let o: ptr<function, Output> = &output;
    return castRay(ray, distance, &output);
}

fn ProcessOutput(ray: Ray, o: Output) -> vec3<f32>
{
    return o.material.color;
}

fn PushRay(location: vec3<f32>, direction: vec3<f32>, colorFilter: vec3<f32>, g: ptr<function, Globals>) -> bool 
{
    var pushed = false;
    if ((*g).rayCount < _rm_MaxRays)
    {
        var ray: Ray;
        ray.location = location + 0.001 * direction;
        ray.direction = direction;
        ray.colorFilter = colorFilter;
        (*g).pendingRays[(*g).rayCount] = ray;
        (*g).rayCount+=1;
        pushed = true;
    }
    return pushed;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32>
{
    //var _rt_pendingRays: array<Ray, 100>;
    var global:  Globals;
    global.rayCount = 0;
    //global.pendingRays = new Array<Ray, 100>();

    //let color = vec3<f32>(0.4);
    PushRay(in.view_pos, normalize(in.view_pos), vec3<f32>(1.0), &global);

    var color = vec3<f32>(0.0);
    for (var i = 0; i < global.rayCount; i+=1)
    {
        var ray: Ray = global.pendingRays[i];
        var o: Output;
        var distance: f32 = getInfinity();
        if (castRay(ray, &distance, &o))
        {
            color += ray.colorFilter * ProcessOutput(ray, o);
        }
    }
    return vec4<f32>(color, 1.0);
}