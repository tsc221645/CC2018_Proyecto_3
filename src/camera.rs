use glam::{Mat4, Vec3, Vec2};
use winit::keyboard::{Key, NamedKey, PhysicalKey};
use winit::event::ElementState;
use winit::event::KeyEvent;

pub struct Camera {
    pub yaw: f32,
    pub pitch: f32,
    pub radius: f32,
    pub target: Vec3,

    pub move_forward: bool,
    pub move_backward: bool,
    pub move_left: bool,
    pub move_right: bool,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            yaw: 0.0,
            pitch: -0.2,
            radius: 20.0,
            target: Vec3::ZERO,

            move_forward: false,
            move_backward: false,
            move_left: false,
            move_right: false,
        }
    }

    pub fn update_from_input(&mut self, dt: f32, mouse_delta: Vec2) {
        // --- Mouse look ---
        let sensitivity = 0.002;
        self.yaw -= mouse_delta.x * sensitivity;
        self.pitch -= mouse_delta.y * sensitivity;

        self.pitch = self.pitch.clamp(-1.4, 1.4);

        // --- WASD movement ---
        let mut dir = Vec3::ZERO;
        let speed = 10.0;

        if self.move_forward {
            dir += Vec3::new(self.yaw.cos(), 0.0, self.yaw.sin());
        }
        if self.move_backward {
            dir -= Vec3::new(self.yaw.cos(), 0.0, self.yaw.sin());
        }
        if self.move_left {
            dir += Vec3::new((self.yaw + std::f32::consts::FRAC_PI_2).cos(), 0.0,
                             (self.yaw + std::f32::consts::FRAC_PI_2).sin());
        }
        if self.move_right {
            dir += Vec3::new((self.yaw - std::f32::consts::FRAC_PI_2).cos(), 0.0,
                             (self.yaw - std::f32::consts::FRAC_PI_2).sin());
        }

        if dir.length() > 0.0 {
            self.target += dir.normalize() * speed * dt;
        }
    }

    pub fn process_key(&mut self, key: &Key, state: ElementState) {
        let pressed = state == ElementState::Pressed;

        match key {
            Key::Character(s) => match s.as_str() {
                "w" | "W" => self.move_forward = pressed,
                "s" | "S" => self.move_backward = pressed,
                "a" | "A" => self.move_left = pressed,
                "d" | "D" => self.move_right = pressed,
                _ => {}
            },

            Key::Named(NamedKey::ArrowUp) => self.move_forward = pressed,
            Key::Named(NamedKey::ArrowDown) => self.move_backward = pressed,
            Key::Named(NamedKey::ArrowLeft) => self.move_left = pressed,
            Key::Named(NamedKey::ArrowRight) => self.move_right = pressed,

            _ => {}
        }
    }

    pub fn view_proj(&self, aspect: f32) -> Mat4 {
        let eye = Vec3::new(
            self.target.x + self.radius * self.yaw.cos() * self.pitch.cos(),
            self.target.y + self.radius * self.pitch.sin(),
            self.target.z + self.radius * self.yaw.sin() * self.pitch.cos(),
        );

        let view = Mat4::look_at_rh(eye, self.target, Vec3::Y);
        let proj = Mat4::perspective_rh(45_f32.to_radians(), aspect, 0.1, 5000.0);

        proj * view
    }
}
