#[derive(Debug, Clone, Copy)]
pub struct PlayerState {
    position: [f32; 3],
    velocity: [f32; 3],
    health: u8,
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

impl PlayerState {
    pub fn update(&mut self, position: [f32; 3], velocity: [f32; 3]) {
        self.position = position;
        self.velocity = velocity;
    }
}
