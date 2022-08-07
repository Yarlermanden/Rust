let _rm_MaxRays: i32 = 3;
fn getInfinity() -> f32 { return 1.0 / 0.0; }
let SPHERE_COUNT: i32 = 10;
let LIGHT_COUNT: i32 = 1;
let BOX_COUNT: i32 = 5;

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

struct Box 
{
    bounds: mat2x4<f32>,
    material: Material,
}

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
    boxes: array<Box, BOX_COUNT>,
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
    //out.view_dir = normalize(mat3x3(camera.inv_view_mat[0].xyz, camera.inv_view_mat[1].xyz, camera.inv_view_mat[2].xyz)*out.view_dir);
    out.view_dir = normalize((camera.inv_view_mat * vec4<f32>(out.view_dir, 0.0)).xyz);
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

fn conditionalSwap(min: ptr<function, f32>, max: ptr<function, f32>) {
    if(*min > *max) {
        let temp: f32 = *min;
        *min = *max;
        *max = temp;
    }
}

fn rayboxIntersection(ray: Ray, box: Box, distance: ptr<function, f32>, o: ptr<function, Output>) -> bool
{
    var hit = false;

    var tmin: f32 = (box.bounds[0].x - ray.location.x) / ray.direction.x;
    var tmax: f32 = (box.bounds[1].x - ray.location.x) / ray.direction.x;
    conditionalSwap(&tmin, &tmax);

    var tymin: f32 = (box.bounds[0].y - ray.location.y) / ray.direction.y;
    var tymax: f32 = (box.bounds[1].y - ray.location.y) / ray.direction.y;
    conditionalSwap(&tymin, &tymax);

    if ((tmin > tymax) || (tymin > tmax)) {
        return false;
    }
    if (tymin > tmin) {
        tmin = tymin;
    }
    if (tymax < tmax) {
        tmax = tymax;
    }

    var tzmin: f32 = (box.bounds[0].z - ray.location.z) / ray.direction.z;
    var tzmax: f32 = (box.bounds[1].z - ray.location.z) / ray.direction.z;
    conditionalSwap(&tzmin, &tzmax);

    if ((tmin > tzmax) || (tzmin > tmax)) {
        return false;
    }
    if (tzmin > tmin) {
        tmin = tzmin;
    }
    if (tzmax < tmax) {
        tmax = tzmax;
    }

    //* -- Possible optimization
    //float tmin, tmax, tymin, tymax, tzmin, tzmax;
    //tmin = (box.bounds[ray.sign[0]].x - ray.location.x) * ray.invDir.x;
    //tmax = (box.bounds[1-ray.sign[0]].x - ray.location.x) * ray.invDir.x;
    //tymin = (box.bounds[ray.sign[1]].y - ray.location.y) * ray.invDir.y;
    //tymax = (box.bounds[1-ray.sign[1]].y - ray.location.y) * ray.invDir.y;
    //if ((tmin > tymax) || (tymin > tmax)) return false; //misses y completely
    //if (tymin > tmin) tmin = tymin;
    //if (tymax < tmax) tmax = tymax;
    //tzmin = (box.bounds[ray.sign[2]].z - ray.location.z) * ray.invDir.z;
    //tzmax = (box.bounds[1-ray.sign[2]].z - ray.location.z) * ray.invDir.z;
    //if ((tmin > tzmax) || (tzmin > tmax)) return false; //misses z completely
    //if (tzmin > tmin) tmin = tzmin;
    //if (tzmax < tmax) tmax = tzmax;
    //*/

    var d: f32 = tmin;
    if(d < 0.0) {
        d = tmax;
        if (d < 0.0) {
            return false;
        }
    }
    //if((*o).lowestTransparency > box.material.transparency) o.lowestTransparency = box.material.transparency;
    if(d >= *distance) {
        return false; //Another object is closer
    }
    //------- It has hit --------
    *distance = d;
    (*o).location = ray.location + d * ray.direction;
    (*o).material = box.material;
    //(*o).normal = normalize((*o).location - .center);

    if(abs((*o).location.x - box.bounds[0].x) < 0.001 || abs((*o).location.x - box.bounds[1].x) < 0.01) {
        if(ray.direction.x > 0.0) {
            (*o).normal = vec3<f32>(-1.0, 0.0, 0.0);
        }
        else {
            (*o).normal = vec3<f32>(1.0, 0.0, 0.0);
        }
    }
    else if(abs((*o).location.y - box.bounds[0].y) < 0.001 || abs((*o).location.y - box.bounds[1].y) < 0.01) {
        if(ray.direction.y > 0.0) {
            (*o).normal = vec3<f32>(0.0, -1.0, 0.0);
        }
        else {
            (*o).normal = vec3<f32>(0.0, 1.0, 0.0);
        }
    }
    else if(abs((*o).location.z - box.bounds[0].z) < 0.001 || abs((*o).location.z - box.bounds[1].z) < 0.01) {
        if(ray.direction.z > 0.0) {
            (*o).normal = vec3<f32>(0.0, 0.0, -1.0);
        }
        else {
            (*o).normal = vec3<f32>(0.0, 0.0, 1.0);
        }
    }
    return true;
}

//output o....
fn castRay(ray: Ray, distance: ptr<function, f32>, o: ptr<function, Output>) -> bool
{
    var hit = false;

    for (var i = 0; i < model.sphere_count; i+=1) {
        hit = raySphereIntersection(ray, model.spheres[i], distance, o) || hit;
    }

    for (var i = 0; i < BOX_COUNT; i+=1) {
        hit = rayboxIntersection(ray, model.boxes[i], distance, o) || hit;
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