use crate::mesh::{load_obj, Vertex};
use wgpu::util::DeviceExt;
use glam::Vec3;

pub struct GpuModel {
    pub vb: wgpu::Buffer,
    pub ib: wgpu::Buffer,
    pub icount: u32,
}

pub struct Scene {
    pub models: Vec<GpuModel>,
    pub orbits: Vec<Vec<Vec3>>,
}

impl Scene {
    pub fn load_models(device: &wgpu::Device) -> Self {
        let paths = [
            "src/models/esfera3.obj",
            "src/models/luna.obj",
            "src/models/ship.obj",
        ];

        let mut models = Vec::new();
        for path in paths {
            let (verts, inds) = load_obj(path);
            let vb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("VB"),
                contents: bytemuck::cast_slice(&verts),
                usage: wgpu::BufferUsages::VERTEX,
            });
            let ib = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("IB"),
                contents: bytemuck::cast_slice(&inds),
                usage: wgpu::BufferUsages::INDEX,
            });
            models.push(GpuModel { vb, ib, icount: inds.len() as u32 });
        }

        // órbitas básicas
        let mut orbits = Vec::new();
        for i in 0..3 {
            let radius = 6.0 + i as f32 * 5.0;
            let orbit_points = (0..=128).map(|j| {
                let a = j as f32 / 128.0 * std::f32::consts::TAU;
                Vec3::new(a.cos() * radius, 0.0, a.sin() * radius)
            }).collect::<Vec<_>>();
            orbits.push(orbit_points);
        }

        Self { models, orbits }
    }
}
