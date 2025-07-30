use std::time::Instant;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PlayerState {
    pub player_id: Uuid,
    position: [f32; 3],
    velocity: [f32; 3],
    health: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct TimedPlayerState {
    player_state: PlayerState,
    time: Instant,
}

impl TimedPlayerState {
    pub fn new(player_state: PlayerState) -> Self {
        Self {
            player_state,
            time: Instant::now(),
        }
    }
}
