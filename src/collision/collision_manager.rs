use nalgebra::Vector3;

use super::bounding_box::BoundingBox;

pub struct CollisionManager {
    map_boxes: Vec<BoundingBox>,
    player_box: BoundingBox,
}

impl CollisionManager {
    /// returns the movement vector after collision calcuations.
    /// Also shifts the player's bounding box to that location.
    pub fn move_player(&mut self, delta: Vector3<f32>) -> Vector3<f32> {
        self.player_box.move_by(delta);
        let colliding_boxes: Vec<&BoundingBox> = self
            .map_boxes
            .iter()
            .filter(|map_box| -> bool { self.player_box.is_colliding_with(map_box) })
            .collect();
        if colliding_boxes.is_empty() {
            delta
        } else {
            self.player_box.move_by(-delta);
            todo!();
        }
    }
}
