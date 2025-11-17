use rand::Rng;

pub fn create_procedural_texture(device: &wgpu::Device, queue: &wgpu::Queue, kind: &str) -> (wgpu::TextureView, wgpu::Sampler) {
    let w = 256;
    let h = 256;
    let data = make_texture_data(kind, w, h);
    let size = wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some(kind),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &data,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * w),
            rows_per_image: Some(h),
        },
        size,
    );
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        address_mode_u: wgpu::AddressMode::Repeat,
        address_mode_v: wgpu::AddressMode::Repeat,
        ..Default::default()
    });
    (view, sampler)
}

fn make_texture_data(kind: &str, w: u32, h: u32) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut data = Vec::with_capacity((w*h*4) as usize);
    for y in 0..h {
        for x in 0..w {
            let u = x as f32 / w as f32;
            let v = y as f32 / h as f32;
            let color = match kind {
                "sun" => sun_color(u,v,&mut rng),
                "rock" => rock_color(u,v,&mut rng),
                "gas" => gas_color(u,v),
                "ice" => ice_color(u,v),
                "ring" => ring_color(u,v),
                _ => [255,255,255,255],
            };
            data.extend_from_slice(&color);
        }
    }
    data
}

fn sun_color(u: f32, v: f32, rng: &mut impl Rng) -> [u8; 4] {
    let r = ((u-0.5).powi(2)+(v-0.5).powi(2)).sqrt() * 2.0;
    let n = rng.gen::<f32>() * 0.2;
    let f = (1.0 - r).max(0.0) + n;
    [(255.0*f) as u8, (180.0*f) as u8, (60.0*f) as u8, 255]
}
fn rock_color(u: f32, v: f32, rng: &mut impl Rng) -> [u8; 4] {
    let n = (rng.gen::<f32>() - 0.5) * 0.4;
    let r = (90.0 + 100.0*(v+n)) as u8;
    let g = (70.0 + 80.0*(u+n)) as u8;
    let b = (60.0 + 40.0*(v+n)) as u8;
    [r,g,b,255]
}
fn gas_color(u: f32, v: f32) -> [u8; 4] {
    let w = ((v*20.0).sin()*0.5+0.5)*0.8+0.2;
    [(100.0+100.0*w) as u8, (150.0+80.0*w) as u8, (200.0-60.0*w) as u8, 255]
}
fn ice_color(u: f32, v: f32) -> [u8; 4] {
    let w = ((u*10.0).cos()*(v*10.0).sin())*0.5+0.5;
    [(150.0*w+80.0) as u8,(200.0*w+55.0) as u8,(255.0*w) as u8,255]
}
fn ring_color(u: f32, v: f32) -> [u8; 4] {
    let d = ((v-0.5)*2.0).abs();
    let fade = (1.0 - d*d).max(0.0);
    [180,180,160,(200.0*fade) as u8]
}