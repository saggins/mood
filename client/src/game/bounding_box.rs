use nalgebra::{Point3, Vector3};

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub top_left: Point3<f32>,
    pub bottom_right: Point3<f32>,
    pub collide_on_top: bool,
}

enum AxisType {
    X,
    Y,
    Z,
}

impl BoundingBox {
    const EPSILON: f32 = 0.0001; // How close you can get before colliding
    const MAX_ITER: u8 = 8; // How many binary search iterations we will do max

    pub fn is_colliding_with(&self, other: &Self) -> bool {
        self.top_left.x < other.bottom_right.x
            && self.bottom_right.x > other.top_left.x
            && self.top_left.y > other.bottom_right.y
            && self.bottom_right.y < other.top_left.y
            && self.top_left.z < other.bottom_right.z
            && self.bottom_right.z > other.top_left.z
    }

    pub fn move_by(&mut self, delta: Vector3<f32>) {
        self.top_left += delta;
        self.bottom_right += delta;
    }

    fn largest_movement_possible_single_axis(
        &self,
        other: &Self,
        delta: f32,
        axis_type: AxisType,
    ) -> f32 {
        if delta.abs() < Self::EPSILON {
            return 0.0;
        }

        let mut t_min = 0.0;
        let mut low = 0.0;
        let mut high = 1.0;

        for _ in 0..Self::MAX_ITER {
            let t = (low + high) / 2.0;
            let move_amount = delta * t;

            let mut test_box = self.clone();
            match axis_type {
                AxisType::X => {
                    test_box.top_left.x += move_amount;
                    test_box.bottom_right.x += move_amount;
                }
                AxisType::Y => {
                    test_box.top_left.y += move_amount;
                    test_box.bottom_right.y += move_amount;
                }
                AxisType::Z => {
                    test_box.top_left.z += move_amount;
                    test_box.bottom_right.z += move_amount;
                }
            }

            if test_box.is_colliding_with(other) {
                high = t;
            } else {
                t_min = t;
                low = t;
            }

            if (high - low).abs() < Self::EPSILON {
                break;
            }
        }

        delta * t_min
    }

    pub fn nearest_non_colliding_delta(&self, other: &Self, delta: Vector3<f32>) -> Vector3<f32> {
        let dx = self.largest_movement_possible_single_axis(other, delta.x, AxisType::X);
        let mut moved = self.clone();
        moved.top_left.x += dx;
        moved.bottom_right.x += dx;

        let dy = moved.largest_movement_possible_single_axis(other, delta.y, AxisType::Y);
        moved.top_left.y += dy;
        moved.bottom_right.y += dy;

        let dz = moved.largest_movement_possible_single_axis(other, delta.z, AxisType::Z);

        Vector3::new(dx, dy, dz)
    }
}
