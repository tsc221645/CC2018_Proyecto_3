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
            ("src/models/sol.obj", 0u32),
            // 1
            ("src/models/mini_planeta_1.obj", 1u32),
            // 2
            ("src/models/nave.obj", 2u32),
            //3
            ("src/models/mini_planeta_2.obj", 3u32),
            //4
            ("src/models/mini_planeta_3.obj", 4u32),
            //5
            ("src/models/huevo_planeta.obj", 5u32),
            //6
            ("src/models/luna.obj", 6u32),
        ];

        let mut models = Vec::new();
        let mut original_vertices = Vec::new();
        let mut dynamic_vertices = Vec::new();

        for (i, (path, planet_id)) in paths.iter().enumerate() {
            let (verts, inds) = load_obj(path, *planet_id);

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
              // órbita del planeta 1
            Scene::make_orbit(12.0),  //orbita planeta 1
            Scene::make_orbit(40.0),  // orbita planeta 2
            Scene::make_orbit(60.0),  // orbita planeta 3
            Scene::make_orbit(75.0),  // orbita planeta huevo
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

    pub fn update(&mut self, time: f32, queue: &wgpu::Queue) {

        //planeta 1 -----------------------------------------------------------
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

        

        
        // nave 2 -----------------------------------------------------------
        let radius4 = 30.0;
        let a4 = time * 0.09;

        let ox2 = a4.cos() * radius4;
        let oz2 = a4.sin() * radius4;

        for (orig, dynv) in self.original_vertices[2]
            .iter()
            .zip(self.dynamic_vertices[2].iter_mut())
        {
            dynv.pos[0] = orig.pos[0] + ox2;
            dynv.pos[2] = orig.pos[2] + oz2;
        }

        queue.write_buffer(
            &self.models[2].vb,
            0,
            bytemuck::cast_slice(&self.dynamic_vertices[2]),
        );
        
       
        // planeta 2 -----------------------------------------------------------
        let radius4 = 40.0;
        let a4 = time * 0.03;

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


        // planeta 3 -----------------------------------------------------------
        let radius4 = 60.0;
        let a4 = time * 0.09;

        let ox4 = a4.cos() * radius4;
        let oz4 = a4.sin() * radius4;

        for (orig, dynv) in self.original_vertices[4]
            .iter()
            .zip(self.dynamic_vertices[4].iter_mut())
        {
            dynv.pos[0] = orig.pos[0] + ox4;
            dynv.pos[2] = orig.pos[2] + oz4;
        }

        queue.write_buffer(
            &self.models[4].vb,
            0,
            bytemuck::cast_slice(&self.dynamic_vertices[4]),
        );

        // planeta huevo  -----------------------------------------------------------
        let radius4 = 75.0;
        let a4 = time * 0.07;

        let ox5 = a4.cos() * radius4;
        let oz5 = a4.sin() * radius4;

        for (orig, dynv) in self.original_vertices[5]
            .iter()
            .zip(self.dynamic_vertices[5].iter_mut())
        {
            dynv.pos[0] = orig.pos[0] + ox5;
            dynv.pos[2] = orig.pos[2] + oz5;
        }

        //orbita de la luna en el planeta huevo
        queue.write_buffer(
            &self.models[5].vb,
            0,
            bytemuck::cast_slice(&self.dynamic_vertices[5]),
        );
        let moon_radius = 10.0;     // distancia alrededor del huevo
        let moon_speed = time * 0.4; // velocidad más alta

        let moon_x = ox5 + moon_speed.cos() * moon_radius;
        let moon_z = oz5 + moon_speed.sin() * moon_radius;

        for (orig, dynv) in self.original_vertices[6]
            .iter()
            .zip(self.dynamic_vertices[6].iter_mut())
        {
            dynv.pos[0] = orig.pos[0] + moon_x;
            dynv.pos[2] = orig.pos[2] + moon_z;
        }

        queue.write_buffer(
            &self.models[6].vb,
            0,
            bytemuck::cast_slice(&self.dynamic_vertices[6]),
        );
    }
}