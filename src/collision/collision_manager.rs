use nalgebra::{Point3, Vector3};

use super::bounding_box::BoundingBox;

#[derive(Debug)]
pub struct CollisionManager {
    pub map_boxes: Vec<BoundingBox>,
    pub player_box: BoundingBox,
}

impl CollisionManager {
    /// returns the movement vector after collision calcuations.
    /// Also shifts the player's bounding box to that location.
    pub fn move_player(&mut self, delta: Vector3<f32>) -> Vector3<f32> {
        let test_box = self.player_box.clone();
        let colliding_boxes: Vec<&BoundingBox> = self
            .map_boxes
            .iter()
            .filter(|map_box| -> bool { test_box.is_colliding_with(map_box) })
            .collect();
        if colliding_boxes.is_empty() {
            self.player_box.move_by(delta);
            delta
        } else {
            let mut new_delta = delta;
            for colliding_box in &colliding_boxes {
                new_delta = self
                    .player_box
                    .nearest_non_colliding_delta(colliding_box, delta);
            }
            self.player_box.move_by(new_delta);
            new_delta
        }
    }
}
