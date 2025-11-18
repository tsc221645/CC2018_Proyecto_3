struct Globals {
    view_proj: mat4x4<f32>,
    time: f32,
    _pad0: vec3<f32>,
    _pad1: vec4<f32>,
    _pad2: vec4<f32>,
};

@group(0) @binding(0) var<uniform> globals: Globals;
@group(0) @binding(1) var planet_texture: texture_2d<f32>;
@group(0) @binding(2) var planet_sampler: sampler;

struct VSIn {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) planet_id: u32, // ID del planeta
};

struct VSOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) planet_id: u32,
};

@vertex fn vs_main(in: VSIn) -> VSOut {
    var out: VSOut;
    out.clip_pos = globals.view_proj * vec4<f32>(in.position, 1.0);
    out.normal = normalize(in.normal);
    out.uv = in.uv;
    out.planet_id = in.planet_id;
    return out;
}

// Funciones de color procedural para cada planeta
fn sun_color(uv: vec2<f32>) -> vec3<f32> {
    // Coordenadas centradas
    let p = (uv - vec2<f32>(0.5)) * 2.0;
    let r = length(p);

    // Brillo central fuertísimo
    let core = 1.5 * max(0.0, 1.0 - r * 1.2);

    // “Plasma” animado
    let t = globals.time * 1.5;
    let wave1 = sin(p.x * 10.0 + t) * 0.4;
    let wave2 = sin(p.y * 12.0 + t * 1.3) * 0.4;
    let wave3 = sin((p.x + p.y) * 8.0 + t * 0.7) * 0.3;

    let plasma = (wave1 + wave2 + wave3) * (1.0 - r);

    // Corona externa (glow)
    let corona = exp(-(r * 3.0)) * 0.8;

    // Color base cálido
    let base = vec3<f32>(1.0, 0.55, 0.15);

    // Mezcla final (más brillante hacia el centro)
    let intensity = core + plasma + corona;

    return base * intensity;
}


fn rock_planet_color(uv: vec2<f32>) -> vec3<f32> {
    let pattern = sin(uv.x * 15.0) * cos(uv.y * 15.0);
    let base = vec3<f32>(0.5, 0.4, 0.3);
    return mix(base, base * 0.7, pattern * 0.5 + 0.5);
}

fn gas_planet_color(uv: vec2<f32>) -> vec3<f32> {
    let bands = sin(uv.y * 20.0) * 0.5 + 0.5;
    let swirl = sin(uv.x * 10.0 - globals.time * 0.5) * 0.3;
    return mix(
        vec3<f32>(0.4, 0.6, 0.8),
        vec3<f32>(0.9, 0.7, 0.4),
        bands + swirl
    );
}

fn ice_planet_color(uv: vec2<f32>) -> vec3<f32> {
    let pattern = cos(uv.x * 10.0) * sin(uv.y * 10.0);
    let base = vec3<f32>(0.8, 0.9, 1.0);
    return mix(base, vec3<f32>(0.5, 0.7, 0.9), abs(pattern) * 0.5);
}

fn egg_planet_color(uv: vec2<f32>) -> vec3<f32> {
    let glow = sin(globals.time) * 0.3 + 0.5;
    let pattern = sin(uv.x * 25.0) * cos(uv.y * 15.0);
    return mix(
        vec3<f32>(0.9, 0.4, 0.2),
        vec3<f32>(1.0, 0.8, 0.3),
        pattern * 0.5 + glow * 0.3
    );
}

fn moon_color(uv: vec2<f32>) -> vec3<f32> {
    let craters = sin(uv.x * 30.0) * sin(uv.y * 30.0) * 0.2;
    let base = vec3<f32>(0.8, 0.8, 0.75);
    return base + vec3<f32>(craters);
}

fn ship_color(uv: vec2<f32>) -> vec3<f32> {
    let metallic = sin(uv.x * 5.0) * 0.2 + 0.8;
    return vec3<f32>(metallic * 0.6, metallic * 0.7, metallic * 0.9);
}

fn get_planet_color(planet_id: u32, uv: vec2<f32>) -> vec3<f32> {
    switch(planet_id) {
        case 0u: { return sun_color(uv); }
        case 1u: { return rock_planet_color(uv); }
        case 2u: { return ship_color(uv); }
        case 3u: { return gas_planet_color(uv); }
        case 4u: { return ice_planet_color(uv); }
        case 5u: { return egg_planet_color(uv); }
        case 6u: { return moon_color(uv); }
        default: { return vec3<f32>(1.0); }
    }
}

@fragment fn fs_main(in: VSOut) -> @location(0) vec4<f32> {
    let color = get_planet_color(in.planet_id, in.uv);
    
    let light = normalize(vec3<f32>(0.4, 0.8, 0.2));
    let diff = max(dot(in.normal, light), 0.0);
    
    // El sol no necesita iluminación, otros sí
    var final_color: vec3<f32>;
    if (in.planet_id == 0u) {
    
        final_color = color * 1.8;
    } else {
        // Otros planetas sí reciben luz
        final_color = color * (0.3 + diff * 0.7);
    }

        
    return vec4<f32>(final_color, 1.0);
}