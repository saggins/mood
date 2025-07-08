use nalgebra::Point3;

pub struct Light {
    pub position: Point3<f32>,
    pub intensity: f32,
    pub color: [f32; 3],
}
