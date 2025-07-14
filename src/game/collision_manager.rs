use nalgebra::Vector3;

use super::bounding_box::BoundingBox;

#[derive(Debug)]
pub struct CollisionManager {
    pub map_boxes: Vec<BoundingBox>,
}

impl CollisionManager {
    /// returns the movement vector after collision calcuations.
    /// Also shifts the player's bounding box to that location.
    pub fn move_player(
        &mut self,
        player_box: &mut BoundingBox,
        velocity: Vector3<f32>,
    ) -> Vector3<f32> {
        let mut test_box = player_box.clone();
        test_box.move_by(velocity);
        let colliding_boxes: Vec<&BoundingBox> = self
            .map_boxes
            .iter()
            .filter(|map_box| -> bool { test_box.is_colliding_with(map_box) })
            .collect();
        if colliding_boxes.is_empty() {
            player_box.move_by(velocity);
            velocity
        } else {
            let mut new_velocity = velocity;
            for colliding_box in &colliding_boxes {
                new_velocity = player_box.nearest_non_colliding_delta(colliding_box, new_velocity);
            }
            player_box.move_by(new_velocity);
            new_velocity
        }
    }
}
