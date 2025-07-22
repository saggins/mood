use nalgebra::{Matrix4, Perspective3, Point3, Vector3};
pub struct Light {
    pub id: u32,
    pub position: Point3<f32>,
    pub intensity: f32,
    pub color: [f32; 3],
}

impl Light {
    pub fn get_view_proj_matrix_for_face(&self, face_index: u32) -> Matrix4<f32> {
        let eye = self.position;
        let (target, up): (Point3<f32>, Vector3<f32>) = match face_index {
            0 => (eye + Vector3::x(), Vector3::y()),  // +X
            1 => (eye - Vector3::x(), Vector3::y()),  // -X
            2 => (eye + Vector3::y(), Vector3::z()),  // +Y
            3 => (eye - Vector3::y(), -Vector3::z()), // -Y
            4 => (eye + Vector3::z(), Vector3::y()),  // +Z
            5 => (eye - Vector3::z(), Vector3::y()),  // -Z
            _ => panic!("Invalid cube face index"),
        };

        let proj = Perspective3::new(1.0, std::f32::consts::FRAC_PI_2, 0.1, 100.0);

        let view = Matrix4::look_at_rh(&eye, &target, &up);

        proj.to_homogeneous() * view
    }
}
