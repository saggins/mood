use image::buffer::EnumerateRows;
use nalgebra::{Point3, Vector3};

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub top_left: Point3<f32>,
    pub bottom_right: Point3<f32>,
    pub collide_on_top: bool,
}

impl BoundingBox {
    /// threshold for collision management
    const EPSILON: f32 = 0.001;
    const MAX_ITER: u8 = 100;
    pub fn is_colliding_with(&self, other: &Self) -> bool {
        self.top_left.x < other.bottom_right.x
            && self.bottom_right.x > other.top_left.x
            && self.top_left.y > other.bottom_right.y
            && self.bottom_right.y < other.top_left.y
            && self.top_left.z < other.bottom_right.z
            && self.bottom_right.z > other.top_left.z
    }

    pub fn move_by(&mut self, delta: Vector3<f32>) {
        self.top_left -= delta;
        self.bottom_right -= delta;
    }

    pub fn nearest_non_colliding_delta(&self, other: &Self, delta: Vector3<f32>) -> Vector3<f32> {
        if !self.is_colliding_with(other) {
            return delta;
        }
        let mut test_box = self.clone();
        test_box.top_left = self.top_left - delta;
        test_box.bottom_right = self.bottom_right - delta;
        if !test_box.is_colliding_with(other) {
            return delta;
        }
        let mut new_delta = delta;
        let mut min = 0.0;
        let mut max = 1.0;
        for _ in 0..Self::MAX_ITER {
            let t = (max + min) / 2.0;
            new_delta = delta * t;
            test_box.top_left = self.top_left - new_delta;
            test_box.bottom_right = self.bottom_right - new_delta;

            if test_box.is_colliding_with(other) {
                max = t;
            } else {
                min = t;
            }

            if max - min < Self::EPSILON {
                break;
            }
        }
        if test_box.is_colliding_with(other) {
            Vector3::zeros()
        } else {
            new_delta
        }
    }
}
