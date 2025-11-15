use glam::{Vec2, Vec3};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: Box::leak(Box::new(wgpu::vertex_attr_array![
                0 => Float32x3, 1 => Float32x3, 2 => Float32x2
            ])),
        }
    }
}

pub fn load_obj(path: &str) -> (Vec<Vertex>, Vec<u32>) {
    let (models, _) = tobj::load_obj(path, &tobj::LoadOptions {
        triangulate: true,
        single_index: true,
        ..Default::default()
    }).expect("Failed to load OBJ");

    let mesh = &models[0].mesh;
    let positions = &mesh.positions;
    let normals = &mesh.normals;
    let texcoords = &mesh.texcoords;

    let mut vertices = Vec::with_capacity(positions.len() / 3);

    for i in 0..positions.len() / 3 {
        let p = [
            positions[i * 3],
            positions[i * 3 + 1],
            positions[i * 3 + 2],
        ];

        let n = if !normals.is_empty() {
            let ni = i * 3;
            [
                *normals.get(ni).unwrap_or(&0.0),
                *normals.get(ni + 1).unwrap_or(&0.0),
                *normals.get(ni + 2).unwrap_or(&1.0),
            ]
        } else {
            [0.0, 1.0, 0.0]
        };

        let uv = if !texcoords.is_empty() {
            let ui = i * 2;
            [
                *texcoords.get(ui).unwrap_or(&0.0),
                *texcoords.get(ui + 1).unwrap_or(&0.0),
            ]
        } else {
            [0.0, 0.0]
        };

        vertices.push(Vertex { pos: p, normal: n, uv });
    }

    (vertices, mesh.indices.clone())
}
