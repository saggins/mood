pub mod camera_uniform;
pub mod light;
pub mod light_uniform;

use nalgebra::{Matrix4, Perspective3, Point3, Vector3};
use winit::{event::ElementState, event_loop::ActiveEventLoop, keyboard::KeyCode};

use crate::collision::collision_manager::CollisionManager;

pub struct Camera {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub near: f32,
    pub far: f32,
    pub is_w_pressed: bool,
    pub is_s_pressed: bool,
    pub is_a_pressed: bool,
    pub is_d_pressed: bool,
    pub yaw: f32,
    pub pitch: f32,
    pub delta: Option<(f32, f32)>,
    pub collision_manager: CollisionManager,
}

impl Camera {
    pub fn get_proj_mat(&self) -> Matrix4<f32> {
        Perspective3::new(self.aspect, self.fovy, self.near, self.far).to_homogeneous()
    }

    pub fn get_view_mat(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.position, &self.target, &self.up)
    }

    pub fn handle_key_held(
        &mut self,
        key: KeyCode,
        state: ElementState,
        event_loop: &ActiveEventLoop,
    ) -> bool {
        match key {
            KeyCode::Escape => {
                event_loop.exit();
                false
            }
            KeyCode::KeyW => {
                self.is_w_pressed = state.is_pressed();
                true
            }
            KeyCode::KeyS => {
                self.is_s_pressed = state.is_pressed();
                true
            }
            KeyCode::KeyD => {
                self.is_d_pressed = state.is_pressed();
                true
            }
            KeyCode::KeyA => {
                self.is_a_pressed = state.is_pressed();
                true
            }
            _ => false,
        }
    }

    pub fn handle_mouse(&mut self, delta: (f64, f64)) {
        let dx = delta.0 as f32;
        let dy = delta.1 as f32;
        self.delta = Some((dx, dy));
    }

    fn camera_shift(&mut self, delta: Vector3<f32>) {
        let valid_delta = self.collision_manager.move_player(delta);
        self.position.x -= valid_delta.x;
        self.position.z -= valid_delta.z;
        self.target.x -= valid_delta.x;
        self.target.z -= valid_delta.z;
    }

    pub fn update_camera(&mut self, move_speed: f32, sensitivity: f32) {
        if let Some(delta) = self.delta {
            self.yaw -= delta.0 * sensitivity;
            self.pitch -= delta.1 * sensitivity;
            self.delta = None;
        }

        let max_pitch = std::f32::consts::FRAC_PI_2 - 0.01;
        self.pitch = self.pitch.clamp(-max_pitch, max_pitch);

        let radius = (self.position - self.target).norm();
        let yaw = self.yaw;
        let pitch = self.pitch;

        self.target.x = self.position.x + radius * pitch.cos() * yaw.sin();
        self.target.y = self.position.y + radius * pitch.sin();
        self.target.z = self.position.z + radius * pitch.cos() * yaw.cos();

        if self.is_w_pressed {
            let mut delta: Vector3<f32> = (self.position - self.target).normalize();
            delta -= delta.dot(&self.up) * self.up;
            delta = delta.normalize() * move_speed;
            self.camera_shift(delta);
        }
        if self.is_s_pressed {
            let mut delta: Vector3<f32> = (self.position - self.target).normalize();
            delta -= delta.dot(&self.up) * self.up;
            delta = delta.normalize() * move_speed;
            self.camera_shift(-delta);
        }
        if self.is_a_pressed {
            let mut delta: Vector3<f32> =
                (self.position - self.target).normalize().cross(&self.up) * move_speed;
            delta -= delta.dot(&self.up) * self.up;
            delta = delta.normalize() * move_speed;
            self.camera_shift(-delta);
        }
        if self.is_d_pressed {
            let mut delta: Vector3<f32> =
                (self.position - self.target).normalize().cross(&self.up) * move_speed;
            delta -= delta.dot(&self.up) * self.up;
            delta = delta.normalize() * move_speed;
            self.camera_shift(delta);
        }
    }
}
