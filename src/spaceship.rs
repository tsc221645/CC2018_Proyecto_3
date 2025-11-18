use crate::mesh::Vertex;
use glam::Vec3;

pub fn generate_spaceship() -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Crear una nave tipo flecha/cohete
    // Cuerpo principal (cilindro adelante)
    let body_length = 3.0;
    let body_radius = 0.5;
    let segments = 12;

    // Punta (cono)
    vertices.push(Vertex {
        pos: [0.0, 0.0, body_length],
        normal: [0.0, 0.0, 1.0],
        uv: [0.5, 1.0],
        planet_id: 2,
    });
    let tip_idx = 0;

    // Cuerpo principal (cilindro)
    for i in 0..=segments {
        let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
        let x = angle.cos() * body_radius;
        let y = angle.sin() * body_radius;

        // Frente del cilindro
        vertices.push(Vertex {
            pos: [x, y, body_length - 0.5],
            normal: [angle.cos(), angle.sin(), 0.0],
            uv: [i as f32 / segments as f32, 0.0],
            planet_id: 2,
        });

        // Atrás del cilindro
        vertices.push(Vertex {
            pos: [x, y, -body_length],
            normal: [angle.cos(), angle.sin(), 0.0],
            uv: [i as f32 / segments as f32, 1.0],
            planet_id: 2,
        });
    }

    let body_front_start = 1;
    let body_back_start = body_front_start + segments + 1;

    // Cono trasero (propulsor)
    vertices.push(Vertex {
        pos: [0.0, 0.0, -body_length - 1.0],
        normal: [0.0, 0.0, -1.0],
        uv: [0.5, 0.0],
        planet_id: 2,
    });
    let back_tip_idx = vertices.len() as u32 - 1;

    // Aletas laterales (3 aletas)
    for wing in 0..3 {
        let wing_angle = (wing as f32 / 3.0) * std::f32::consts::TAU;
        let wing_x = wing_angle.cos() * body_radius * 1.5;
        let wing_y = wing_angle.sin() * body_radius * 1.5;

        // Base de aleta
        vertices.push(Vertex {
            pos: [wing_x, wing_y, -body_length * 0.5],
            normal: [wing_x, wing_y, 0.0],
            uv: [0.0, 0.0],
            planet_id: 2,
        });

        // Punta de aleta
        vertices.push(Vertex {
            pos: [wing_x * 2.0, wing_y * 2.0, -body_length * 0.7],
            normal: [wing_x, wing_y, 0.2],
            uv: [1.0, 1.0],
            planet_id: 2,
        });
    }

    let wing_start = vertices.len() as u32 - 6;

    // Índices - Cono frontal
    for i in 0..segments {
        indices.push(tip_idx);
        indices.push(body_front_start as u32 + i as u32 * 2);
        indices.push(body_front_start as u32 + ((i + 1) % (segments + 1)) as u32 * 2);
    }

    // Índices - Cuerpo cilíndrico
    for i in 0..segments {
        let front1 = body_front_start as u32 + i as u32 * 2;
        let front2 = body_front_start as u32 + ((i + 1) % (segments + 1)) as u32 * 2;
        let back1 = body_back_start as u32 + i as u32 * 2;
        let back2 = body_back_start as u32 + ((i + 1) % (segments + 1)) as u32 * 2;

        // Triángulo 1
        indices.push(front1);
        indices.push(back1);
        indices.push(front2);

        // Triángulo 2
        indices.push(front2);
        indices.push(back1);
        indices.push(back2);
    }

    // Índices - Cono trasero
    for i in 0..segments {
        indices.push(back_tip_idx);
        indices.push(body_back_start as u32 + ((i + 1) % (segments + 1)) as u32 * 2);
        indices.push(body_back_start as u32 + i as u32 * 2);
    }

    // Índices - Aletas
    for wing in 0..3 {
        let base = wing_start + wing as u32 * 2;
        let tip = base + 1;
        let next_base = wing_start + ((wing + 1) % 3) as u32 * 2;

        indices.push(base);
        indices.push(tip);
        indices.push(next_base);
    }

    (vertices, indices)
}