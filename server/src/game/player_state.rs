use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PlayerState {
    pub player_id: Uuid,
    position: [f32; 3],
    velocity: [f32; 3],
    health: u8,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            player_id: Uuid::new_v4(),
            position: [0.0, 0.0, 0.0],
            velocity: [0.0, 0.0, 0.0],
            health: 100,
        }
    }
}

impl PlayerState {
    pub fn update(&mut self, position: [f32; 3], velocity: [f32; 3]) {
        self.position = position;
        self.velocity = velocity;
    }
}
