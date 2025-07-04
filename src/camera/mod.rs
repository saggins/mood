pub mod camera_uniform;
use nalgebra::{Matrix4, Perspective3, Point3, Vector3};
use winit::{
    event::{ElementState, KeyEvent},
    keyboard::KeyCode,
};

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
}

impl Camera {
    pub fn get_proj_view_mat(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(&self.position, &self.target, &self.up);
        let proj = Perspective3::new(self.aspect, self.fovy, self.near, self.far).to_homogeneous();

        proj * view
    }

    pub fn handle_key_held(&mut self, key: KeyCode, state: ElementState) -> bool {
        match key {
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

    pub fn update_camera(&mut self, move_speed: f32, turn_speed: f32) {
        let direction = Vector3::new(self.yaw.sin(), 0.0, self.yaw.cos()).normalize();
        if self.is_w_pressed {
            self.position += direction * move_speed;
        }
        if self.is_s_pressed {
            self.position -= direction * move_speed;
        }
        if self.is_a_pressed {
            self.yaw += turn_speed;
        }
        if self.is_d_pressed {
            self.yaw -= turn_speed;
        }

        self.target = self.position + direction;
    }
}
