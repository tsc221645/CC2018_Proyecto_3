struct Globals {
    view_proj: mat4x4<f32>,
    time: f32,       // +4
    _pad0: vec3<f32>,// +12 = 80
    _pad1: vec4<f32>,// +16 = 96
    _pad2: vec4<f32>,// +16 = 112 ✔ MATCHES RUST
};
@group(0) @binding(0)
var<uniform> globals: Globals;

struct VSIn {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VSOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

@vertex
fn vs_main(in: VSIn) -> VSOut {
    var out: VSOut;
    out.clip_pos = globals.view_proj * vec4<f32>(in.position, 1.0);
    out.normal = normalize(in.normal);
    out.uv = in.uv;
    return out;
}

@fragment
fn fs_main(in: VSOut) -> @location(0) vec4<f32> {
    let light = normalize(vec3<f32>(0.4, 0.8, 0.2));
    let diff = max(dot(in.normal, light), 0.0);
    return vec4<f32>(0.4 + diff, 0.3 + diff * 0.5, 1.0 - diff, 1.0);
}
