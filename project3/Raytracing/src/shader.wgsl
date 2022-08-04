struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
    inv_view_mat: mat4x4<f32>,
    inv_proj_mat: mat4x4<f32>,
}
@group(0) @binding(0)
var<uniform> camera: Camera;

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





@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.3, 0.3, 1.0);
}