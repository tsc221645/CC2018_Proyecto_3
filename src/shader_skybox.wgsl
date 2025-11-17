struct Globals {
    view_proj: mat4x4<f32>,
    time: f32,
    _pad0: vec3<f32>,
    _pad1: vec4<f32>,
    _pad2: vec4<f32>,
};

@group(0) @binding(0) var<uniform> globals: Globals;

@vertex fn vs_main(@builtin(vertex_index) idx: u32) -> @builtin(position) vec4<f32> {
    // Generamos un fullscreen quad
    var uv = vec2<f32>(f32(idx & 1u), f32((idx >> 1u) & 1u)) * 2.0;
    return vec4<f32>(uv * 2.0 - 1.0, 1.0, 1.0);
}

// Función de ruido pseudo-aleatorio de calidad
fn hash31(p: vec3<f32>) -> f32 {
    var p3 = fract(p * 0.1031);
    p3 = p3 + dot(p3, p3.yzx + 19.19);
    return fract((p3.x + p3.y) * p3.z);
}

fn hash33(p: vec3<f32>) -> vec3<f32> {
    var p3 = fract(p * vec3<f32>(0.1031, 0.1030, 0.0973));
    p3 = p3 + dot(p3, p3.yzx + 19.19);
    return fract((p3.xxy + vec3<f32>(p3.z, p3.z, p3.x)) * p3.yzx);
}

// Generar estrellas
fn star_field(dir: vec3<f32>) -> vec3<f32> {
    let cell_size = 64.0;
    let grid_pos = floor(dir * cell_size);
    let local_pos = fract(dir * cell_size);
    
    var total_light = vec3<f32>(0.0);
    
    // Verificar celdas vecinas
    for (var z: i32 = -1; z <= 1; z = z + 1) {
        for (var y: i32 = -1; y <= 1; y = y + 1) {
            for (var x: i32 = -1; x <= 1; x = x + 1) {
                let cell = grid_pos + vec3<f32>(f32(x), f32(y), f32(z));
                let rnd = hash31(cell);
                
                // Si el valor es alto, colocar una estrella
                if (rnd > 0.92) {
                    let star_color = hash33(cell);
                    let star_local_pos = fract(hash33(cell * 2.0));
                    let dist = distance(local_pos, star_local_pos);
                    
                    // Crear un pico de estrella suave
                    if (dist < 0.08) {
                        let brightness = exp(-dist * dist * 200.0);
                        let color = mix(
                            vec3<f32>(1.0, 1.0, 1.0),
                            vec3<f32>(
                                0.8 + star_color.x * 0.2,
                                0.8 + star_color.y * 0.2,
                                0.8 + star_color.z * 0.2
                            ),
                            0.3
                        );
                        total_light += color * brightness * 0.8;
                    }
                }
            }
        }
    }
    
    return total_light;
}

@fragment fn fs_main(
    @builtin(position) pos: vec4<f32>
) -> @location(0) vec4<f32> {
    // Fondo completamente negro
    var color = vec3<f32>(0.0, 0.0, 0.0);
    
    // Extraer la dirección de vista del view_proj
    // Normalizar coordenadas de pantalla a [-1, 1]
    let uv = (pos.xy / vec2<f32>(800.0, 600.0)) * 2.0 - 1.0;
    
    // Crear un rayo de vista simple basado en las coordenadas UV
    // Esto hace que las estrellas se muevan con la cámara
    let aspect = 800.0 / 600.0;
    var ray_dir = vec3<f32>(uv.x * aspect, uv.y, -1.0);
    ray_dir = normalize(ray_dir);
    
    // Obtener estrellas en esa dirección
    let stars = star_field(ray_dir);
    color = color + stars;
    
    return vec4<f32>(color, 1.0);
}