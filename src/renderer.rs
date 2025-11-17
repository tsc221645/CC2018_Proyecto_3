use wgpu::util::DeviceExt;
use crate::mesh::Vertex;
use glam::Mat4;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Globals {
    pub view_proj: [[f32; 4]; 4],
    pub time: f32,
    pub _pad0: [f32; 3],
    pub _pad1: [f32; 4],
    pub _pad2: [f32; 4],
    pub _pad3: [f32; 4],
}

pub struct Renderer {
    pub globals_buf: wgpu::Buffer,
    pub globals_bg: wgpu::BindGroup,
    pub pipeline: wgpu::RenderPipeline,
    pub orbit_pipeline: wgpu::RenderPipeline,
    pub dummy_texture: wgpu::TextureView,
    pub dummy_sampler: wgpu::Sampler,
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

        // Crear texturas dummy (placeholder)
        let dummy_texture = Self::create_dummy_texture(device);
        let dummy_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let globals_bg_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Globals Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let globals_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &globals_bg_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: globals_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&dummy_texture),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&dummy_sampler),
                },
            ],
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

        Self {
            globals_buf,
            globals_bg,
            pipeline,
            orbit_pipeline,
            dummy_texture,
            dummy_sampler,
        }
    }

    fn create_dummy_texture(device: &wgpu::Device) -> wgpu::TextureView {
        let size = wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Dummy Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        texture.create_view(&wgpu::TextureViewDescriptor::default())
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
                    planet_id: 0, // Las órbitas no necesitan un planeta específico
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