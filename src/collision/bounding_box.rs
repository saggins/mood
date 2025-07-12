use nalgebra::{Point3, Vector3};

pub struct BoundingBox {
    top_left: Point3<f32>,
    bottom_right: Point3<f32>,
    pub collide_on_top: bool,
}

impl BoundingBox {
    pub fn is_colliding_with(&self, other: &Self) -> bool {
        self.top_left.x <= other.bottom_right.x
            && self.bottom_right.x >= other.top_left.x
            && self.top_left.y <= other.bottom_right.y
            && self.bottom_right.y >= other.bottom_right.y
            && self.top_left.z <= other.bottom_right.z
            && self.bottom_right.z >= other.bottom_right.z
    }

    pub fn move_by(&mut self, delta: Vector3<f32>) {
        self.top_left += delta;
        self.bottom_right += delta;
    }

    pub fn nearest_non_colliding_delta(&self, other: &Self) -> Vector3<f32> {
        todo!();
    }
}
