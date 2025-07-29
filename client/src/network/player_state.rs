use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PlayerState {
    pub player_id: Uuid,
    position: [f32; 3],
    velocity: [f32; 3],
    health: u8,
}
