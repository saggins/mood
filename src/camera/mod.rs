pub mod camera_uniform;
pub mod light;
pub mod light_uniform;

use nalgebra::{Matrix4, Perspective3, Point3, Vector3};

pub struct Camera {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn get_proj_mat(&self) -> Matrix4<f32> {
        Perspective3::new(self.aspect, self.fovy, self.near, self.far).to_homogeneous()
    }

    pub fn get_view_mat(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.position, &self.target, &self.up)
    }

    pub fn rotate_camera(&mut self, pitch: f32, yaw: f32) {
        let radius = (self.position - self.target).norm();

        self.target.x = self.position.x + radius * pitch.cos() * yaw.sin();
        self.target.y = self.position.y + radius * pitch.sin();
        self.target.z = self.position.z + radius * pitch.cos() * yaw.cos();
    }

    pub fn move_camera(&mut self, delta: Vector3<f32>) {
        self.position += delta;
        self.target += delta;
    }
}
