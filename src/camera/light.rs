use nalgebra::{Matrix4, Perspective3, Point3, Vector3};
pub struct Light {
    pub id: u32,
    pub position: Point3<f32>,
    pub intensity: f32,
    pub color: [f32; 3],
}
