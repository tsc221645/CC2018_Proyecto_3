mod renderer;
mod camera;
mod mesh;
mod orbit;
mod scene;
mod procedural_texture;

use std::sync::Arc;
use winit::{event::*, event_loop::EventLoop};
use pollster::block_on;
use glam::Vec2;
use renderer::{Renderer, Globals};
use camera::Camera;
use scene::Scene;
use camera::CollisionSphere;

fn main() {
    block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new().unwrap();

    let window = Arc::new(
        event_loop
            .create_window(
                winit::window::WindowAttributes::default()
                    .with_title("SolarSoft GPU Procedural + OBJ"),
            )
            .unwrap(),
    );

    let instance = wgpu::Instance::default();
    let surface = instance.create_surface(window.clone()).unwrap();
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

    let size = window.inner_size();
    let format = surface.get_capabilities(&adapter).formats[0];

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        desired_maximum_frame_latency: 1,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: vec![],
    };
    surface.configure(&device, &config);

    let renderer = Renderer::new(&device, format, size.width, size.height).await;
    let mut cam = Camera::new();
    let mut scene = Scene::load_models(&device);

    let mut time = 0.0f32;
    let mut last = std::time::Instant::now();
    let mut mouse_delta = Vec2::ZERO;

    event_loop
        .run(move |event, elwt| match event {

            /* ---------- Cerrar ventana ---------- */
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => elwt.exit(),

            /* ---------- Mouse movement ---------- */
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    mouse_delta += Vec2::new(delta.0 as f32, delta.1 as f32);
                }
                DeviceEvent::MouseWheel { delta } => {
                    let zoom_speed = 0.15; 
                    let scroll = match delta {
                        MouseScrollDelta::LineDelta(_, y) => y * zoom_speed,
                        MouseScrollDelta::PixelDelta(p) => (p.y as f32) * zoom_speed,
                    };
                    cam.radius = (cam.radius - scroll).clamp(3.0, 500.0);
                }
                _ => {}
            },

            /* ---------- Keyboard movement ---------- */
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { event: KeyEvent { logical_key, state, .. }, .. },
                ..
            } => {
                cam.process_key(&logical_key, state);
            }

            /* ---------- Pedimos redibujar continuamente ---------- */
            Event::AboutToWait => {
                window.clone().request_redraw();
            }

            /* ---------- Render ---------- */
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                let now = std::time::Instant::now();
                let dt = (now - last).as_secs_f32();
                last = now;
                time += dt;
                scene.update(time, &queue);

                // Convertir posiciones de planetas a esferas de colisión
                let collision_spheres: Vec<CollisionSphere> = scene.planet_positions
                    .iter()
                    .map(|(pos, radius)| CollisionSphere { center: *pos, radius: *radius })
                    .collect();

                cam.update_from_input(dt, mouse_delta, &collision_spheres);
                mouse_delta = Vec2::ZERO;

                let frame = match surface.get_current_texture() {
                    Ok(f) => f,
                    Err(_) => {
                        surface.configure(&device, &config);
                        return;
                    }
                };

                let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let aspect = config.width as f32 / config.height as f32;

                let globals = Globals {
                    view_proj: cam.view_proj(aspect).to_cols_array_2d(),
                    time,
                    _pad0: [0.0; 3],
                    _pad1: [0.0; 4],
                    _pad2: [0.0; 4],
                    _pad3: [0.0; 4],
                };
                queue.write_buffer(&renderer.globals_buf, 0, bytemuck::bytes_of(&globals));

                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                {
                    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Main Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 1.0,
                                }),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                            view: &renderer.depth_texture,
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Clear(1.0),
                                store: wgpu::StoreOp::Store,
                            }),
                            stencil_ops: None,
                        }),
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });

                    /* ------ Dibujar skybox/fondo con estrellas ------ */
                    renderer.draw_skybox(&mut pass);

                    /* ------ Dibujar planetas / modelos ------ */
                    for model in &scene.models {
                        renderer.draw_mesh(&mut pass, &model.vb, &model.ib, model.icount);
                    }

                    /* ------ Dibujar órbitas ------ */
                    renderer.draw_orbits(&mut pass, &device, &scene.orbits);
                }

                queue.submit(Some(encoder.finish()));
                frame.present();
            }

            _ => {}
        })
        .unwrap();
}