use glam::{Mat4, Vec3, Vec2};
use winit::keyboard::{Key, NamedKey};
use winit::event::ElementState;

pub struct Camera {
    pub yaw: f32,
    pub pitch: f32,
    pub radius: f32,
    pub target: Vec3,

    pub move_forward: bool,
    pub move_backward: bool,
    pub move_left: bool,
    pub move_right: bool,
    
    // Control de nave del jugador
    pub ship_move_forward: bool,
    pub ship_move_backward: bool,
    pub ship_turn_left: bool,
    pub ship_turn_right: bool,
    pub ship_turn_up: bool,
    pub ship_turn_down: bool,
    
    // Modo de vista
    pub ship_view: bool, // true = vista de nave, false = vista libre
}

// Estructura para representar esferas de colisión (planetas)
pub struct CollisionSphere {
    pub center: Vec3,
    pub radius: f32,
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
            
            ship_move_forward: false,
            ship_move_backward: false,
            ship_turn_left: false,
            ship_turn_right: false,
            ship_turn_up: false,
            ship_turn_down: false,
            
            ship_view: false,
        }
    }

    pub fn update_from_input(&mut self, dt: f32, mouse_delta: Vec2, planets: &[CollisionSphere]) {
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
            let new_target = self.target + dir.normalize() * speed * dt;
            
            // Verificar colisiones antes de mover
            if !self.check_collision(new_target, planets) {
                self.target = new_target;
            }
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
                "v" | "V" => if pressed { self.ship_view = !self.ship_view }, // Cambiar vista con V
                _ => {}
            },

            Key::Named(NamedKey::ArrowUp) => self.ship_turn_up = pressed,
            Key::Named(NamedKey::ArrowDown) => self.ship_turn_down = pressed,
            Key::Named(NamedKey::ArrowLeft) => self.ship_turn_left = pressed,
            Key::Named(NamedKey::ArrowRight) => self.ship_turn_right = pressed,

            _ => {}
        }
    }

    // Detectar colisión con esferas (planetas)
    fn check_collision(&self, new_target: Vec3, planets: &[CollisionSphere]) -> bool {
        // Radio de colisión de la cámara
        let camera_radius = 2.0;
        
        for planet in planets {
            let dist_to_planet = new_target.distance(planet.center);
            if dist_to_planet < (planet.radius + camera_radius) {
                return true; // Colisión detectada
            }
        }
        
        false // Sin colisión
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

    // Obtener posición de los ojos de la cámara
    pub fn get_eye_pos(&self) -> Vec3 {
        Vec3::new(
            self.target.x + self.radius * self.yaw.cos() * self.pitch.cos(),
            self.target.y + self.radius * self.pitch.sin(),
            self.target.z + self.radius * self.yaw.sin() * self.pitch.cos(),
        )
    }

    // Vista en primera persona desde la nave
    pub fn view_proj_from_ship(&self, ship_pos: Vec3, ship_rot: (f32, f32), aspect: f32) -> Mat4 {
        let offset_distance = 3.0;
        
        // Crear dirección hacia atrás basada en rotación de nave
        let backward = Vec3::new(
            ship_rot.0.cos() * ship_rot.1.cos(),
            ship_rot.1.sin(),
            ship_rot.0.sin() * ship_rot.1.cos(),
        );
        
        // Posición de la cámara: detrás de la nave
        let eye = ship_pos - backward * offset_distance + Vec3::new(0.0, 1.0, 0.0);
        
        // La cámara mira hacia adelante de la nave
        let target = ship_pos + backward * 100.0;
        
        let view = Mat4::look_at_rh(eye, target, Vec3::Y);
        let proj = Mat4::perspective_rh(45_f32.to_radians(), aspect, 0.1, 5000.0);

        proj * view
    }

    // Actualizar la nave del jugador basándose en rotación de flechas
    pub fn update_player_ship(&self, dt: f32, ship_pos: &mut Vec3, ship_rot: &mut (f32, f32)) {
        let ship_speed = 40.0;
        let rotation_speed = 2.0;

        // Rotar nave
        if self.ship_turn_left {
            ship_rot.0 += rotation_speed * dt;
        }
        if self.ship_turn_right {
            ship_rot.0 -= rotation_speed * dt;
        }
        if self.ship_turn_up {
            ship_rot.1 = (ship_rot.1 + rotation_speed * dt).min(std::f32::consts::FRAC_PI_2);
        }
        if self.ship_turn_down {
            ship_rot.1 = (ship_rot.1 - rotation_speed * dt).max(-std::f32::consts::FRAC_PI_2);
        }

        // Mover nave en dirección de rotación
        let forward = Vec3::new(
            ship_rot.0.cos() * ship_rot.1.cos(),
            ship_rot.1.sin(),
            ship_rot.0.sin() * ship_rot.1.cos(),
        );

        *ship_pos += forward * ship_speed * dt;
    }
}