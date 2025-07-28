use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PlayerState {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub health: u8,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            velocity: [0.0, 0.0, 0.0],
            health: 100,
        }
    }
}
