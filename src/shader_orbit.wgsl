struct Globals {
    view_proj: mat4x4<f32>,
    time: f32,
    _pad0: vec3<f32>,
    _pad1: vec4<f32>,
    _pad2: vec4<f32>,
};
@group(0) @binding(0)
var<uniform> globals: Globals;

struct VSIn { @location(0) position: vec3<f32> };

@vertex
fn vs_main(in: VSIn) -> @builtin(position) vec4<f32> {
    return globals.view_proj * vec4<f32>(in.position, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.8, 0.8, 1.0, 1.0);
}
