use glam::Vec3;

pub fn generate_orbit(radius: f32, segments: u32) -> Vec<Vec3> {
    let mut pts = Vec::new();
    for i in 0..=segments {
        let t = i as f32 / segments as f32 * std::f32::consts::TAU;
        pts.push(Vec3::new(t.cos() * radius, 0.0, t.sin() * radius));
    }
    pts
}
