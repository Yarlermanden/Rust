let _rm_MaxRays: i32 = 3;
fn getInfinity() -> f32 { return 1.0 / 0.0; }
let SPHERE_COUNT: i32 = 10;
let LIGHT_COUNT: i32 = 1;

struct Ray
{
    location: vec3<f32>,
    direction: vec3<f32>,
    colorFilter: vec3<f32>,
};

struct Material
{
    color: vec3<f32>,
    padding: f32,

    //PhongLighting
    I_aK_a: f32, //I_a * K_a
    diffuse: f32, //I Kdf
    Ks: f32, //specular reflectance
    exp: f32, //specular exponent
};

struct Output
{
    location: vec3<f32>,
    normal: vec3<f32>,
    refractPoint: vec3<f32>,
    refractDirection: vec3<f32>,
    material: Material,
};

struct Light 
{
    location: vec3<f32>,
    padding: f32,
    color: vec3<f32>,
    padding2: f32,
}

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
    sphere_count: i32,
    padding2: f32,
    padding3: f32,
    spheres: array<Sphere, SPHERE_COUNT>,
    lights: array<Light, LIGHT_COUNT>,
}
@group(1) @binding(0)
var<uniform> model: Model;

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) view_pos: vec3<f32>,
    @location(1) view_dir: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);

    var viewPos = camera.inv_view_mat *  camera.inv_proj_mat * out.clip_position;
    out.view_pos = viewPos.xyz / viewPos.w;

    var viewDir = normalize(camera.inv_proj_mat * out.clip_position);
    out.view_dir = viewDir.xyz / viewDir.w;
    out.view_dir = normalize(mat3x3(camera.inv_view_mat[0].xyz, camera.inv_view_mat[1].xyz, camera.inv_view_mat[2].xyz)*out.view_dir);
    return out;
}


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
    var hit = false;

    for (var i = 0; i < model.sphere_count; i+=1) {
        hit = raySphereIntersection(ray, model.spheres[i], distance, o) || hit;
    }

    return hit;
}

fn castRay1(ray: Ray, distance: ptr<function, f32>) -> bool
{
    var output: Output;
    //let o: ptr<function, Output> = &output;
    return castRay(ray, distance, &output);
}

fn PhongLighting(ray: Ray, o: Output) -> vec3<f32> {
    let R_ambient: vec3<f32> = o.material.I_aK_a * o.material.color; //Ia * Ka * color
    var col: vec3<f32> = R_ambient;

    for(var i = 0; i < LIGHT_COUNT; i+=1) {
        let L: vec3<f32> = normalize(model.lights[i].location - o.location); //light direction
        let R_diffuse: vec3<f32> = model.lights[i].color * o.material.diffuse * max(dot(o.normal, L), 0.0) * o.material.color; //I_light * Kd * (N•L) * Color

        let V: vec3<f32> = -ray.direction; //view direction
        let H: vec3<f32> = normalize(L+V); //halfway vector between light direction and view direction
        let R_specular: vec3<f32> = model.lights[i].color * o.material.Ks * pow(max(dot(o.normal, H), 0.0), o.material.exp); //I_light * Ks * (N•H)^exp

        //let distance: f32 = length(model.lights[i].location - o.location);
        //let attenuation: f32 = 1.0/pow(distance,2.0);

        //col += o.invAmountOfShadow[i] * attenuation * (R_diffuse + R_specular);
        //col += attenuation * (R_diffuse + R_specular);
        col += R_diffuse + R_specular;
    }
    return col;
}

fn ProcessOutput(ray: Ray, o: Output) -> vec3<f32>
{
    //return o.material.color;
    return PhongLighting(ray, o);
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
    var global:  Globals;
    global.rayCount = 0;

    //PushRay(in.view_pos, normalize(in.view_pos), vec3<f32>(1.0), &global);
    PushRay(in.view_pos, in.view_dir, vec3<f32>(1.0), &global);

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