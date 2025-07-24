pub enum InputPacketType {
    GameState,
    PlayerJoin,
    PlayerLeave,
    PlayerInput,
}

pub enum OutputPacketType {
    Success,
    Failed,
    Data,
}
