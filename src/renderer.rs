use wgpu::util::DeviceExt;
use crate::mesh::Vertex;
use glam::Mat4;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Globals {
    pub view_proj: [[f32; 4]; 4], // 64 bytes
    pub time: f32,                // +4
    pub _pad0: [f32; 3],          // +12 => 80
    pub _pad1: [f32; 4],          // +16 => 96
    pub _pad2: [f32; 4],          // +16 => 112
    pub _pad3: [f32; 4],          // +16 => 128  <-- EXTRA PADDING
}

pub struct Renderer {
    pub globals_buf: wgpu::Buffer,
    pub globals_bg: wgpu::BindGroup,
    pub pipeline: wgpu::RenderPipeline,
    pub orbit_pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub async fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let globals_init = Globals {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
            time: 0.0,
            _pad0: [0.0; 3],
            _pad1: [0.0; 4],
            _pad2: [0.0; 4],
            _pad3: [0.0; 4],
        };

        let globals_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Globals"),
            contents: bytemuck::bytes_of(&globals_init),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let globals_bg_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Globals Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let globals_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &globals_bg_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: globals_buf.as_entire_binding(),
            }],
            label: Some("Globals BG"),
        });

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader_textured.wgsl"));
        let orbit_shader = device.create_shader_module(wgpu::include_wgsl!("shader_orbit.wgsl"));

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&globals_bg_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Main Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::layout()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let orbit_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Orbit Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &orbit_shader,
                entry_point: "vs_main",
                buffers: &[Vertex::layout()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &orbit_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineStrip,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self { globals_buf, globals_bg, pipeline, orbit_pipeline }
    }

    pub fn draw_mesh<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        vb: &'a wgpu::Buffer,
        ib: &'a wgpu::Buffer,
        ic: u32,
    ) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.globals_bg, &[]);
        pass.set_vertex_buffer(0, vb.slice(..));
        pass.set_index_buffer(ib.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..ic, 0, 0..1);
    }

    pub fn draw_orbits<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        device: &wgpu::Device,
        orbits: &Vec<Vec<glam::Vec3>>,
    ) {
        pass.set_pipeline(&self.orbit_pipeline);
        pass.set_bind_group(0, &self.globals_bg, &[]);

        for orbit in orbits {
            let verts: Vec<Vertex> = orbit
                .iter()
                .map(|p| Vertex {
                    pos: p.to_array(),
                    normal: [0.0; 3],
                    uv: [0.0; 2],
                })
                .collect();

            let vb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Orbit VB"),
                contents: bytemuck::cast_slice(&verts),
                usage: wgpu::BufferUsages::VERTEX,
            });

            pass.set_vertex_buffer(0, vb.slice(..));
            pass.draw(0..verts.len() as u32, 0..1);
        }
    }
}
