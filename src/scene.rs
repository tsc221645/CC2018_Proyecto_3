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

    // 🔥 movimiento dinámico
    original_vertices: Vec<Vec<Vertex>>,
    dynamic_vertices: Vec<Vec<Vertex>>,
}

impl Scene {
    pub fn load_models(device: &wgpu::Device) -> Self {
        let paths = [
            // 0
            "src/models/sol.obj",
            // 1
            "src/models/mini_planeta_1.obj",
            // 2
            "src/models/ship.obj",
            //3
            "src/models/mini_planeta_2.obj",
        ];

        let mut models = Vec::new();
        let mut original_vertices = Vec::new();
        let mut dynamic_vertices = Vec::new();

        for (i, path) in paths.iter().enumerate() {
            let (verts, inds) = load_obj(path);

            // guardamos copias para animación
            original_vertices.push(verts.clone());
            dynamic_vertices.push(verts.clone());

            let vb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("VB"),
                contents: bytemuck::cast_slice(&dynamic_vertices[i]),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });

            let ib = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("IB"),
                contents: bytemuck::cast_slice(&inds),
                usage: wgpu::BufferUsages::INDEX,
            });

            models.push(GpuModel {
                vb,
                ib,
                icount: inds.len() as u32,
            });
        }

        // ⭐ órbitas independientes
        let orbits = vec![
            Scene::make_orbit(6.0),   // órbita del planeta 1
            Scene::make_orbit(12.0),  // órbita planeta 2 / nave si quieres
            Scene::make_orbit(20.0),  // órbita extra
        ];

        Self {
            models,
            orbits,
            original_vertices,
            dynamic_vertices,
        }
    }

    /// Genera una órbita circular
    fn make_orbit(radius: f32) -> Vec<Vec3> {
        (0..=128)
            .map(|j| {
                let a = j as f32 / 128.0 * std::f32::consts::TAU;
                Vec3::new(a.cos() * radius, 0.0, a.sin() * radius)
            })
            .collect()
    }

    /// 🔥 ANIMAR planetas aquí mismo, sin tocar renderer ni shaders
    pub fn update(&mut self, time: f32, queue: &wgpu::Queue) {

        // ============================================================
        // ⭐ PLANETA 1 ORBITANDO
        // ============================================================
        let radius = 12.0;
        let angle = time * 0.3;
        let ox = angle.cos() * radius;
        let oz = angle.sin() * radius;

        for (orig, dynv) in self.original_vertices[1]
            .iter()
            .zip(self.dynamic_vertices[1].iter_mut())
        {
            dynv.pos[0] = orig.pos[0] + ox;
            dynv.pos[2] = orig.pos[2] + oz;
        }

        queue.write_buffer(
            &self.models[1].vb,
            0,
            bytemuck::cast_slice(&self.dynamic_vertices[1]),
        );

        // ============================================================
        // ⭐ NAVE MOVIENDO EN ONDA (ejemplo)
        // ============================================================
        let lift = (time * 1.5).sin() * 1.5;

        for (orig, dynv) in self.original_vertices[2]
            .iter()
            .zip(self.dynamic_vertices[2].iter_mut())
        {
            dynv.pos[1] = orig.pos[1] + lift;
        }

        queue.write_buffer(
            &self.models[2].vb,
            0,
            bytemuck::cast_slice(&self.dynamic_vertices[2]),
        );

        let radius4 = 40.0;
        let a4 = time * 0.09;

        let ox3 = a4.cos() * radius4;
        let oz3 = a4.sin() * radius4;

        for (orig, dynv) in self.original_vertices[3]
            .iter()
            .zip(self.dynamic_vertices[3].iter_mut())
        {
            dynv.pos[0] = orig.pos[0] + ox3;
            dynv.pos[2] = orig.pos[2] + oz3;
        }

        queue.write_buffer(
            &self.models[3].vb,
            0,
            bytemuck::cast_slice(&self.dynamic_vertices[3]),
        );
    }
}
