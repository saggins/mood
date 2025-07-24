use client::Game;
use winit::error::EventLoopError;

fn main() -> Result<(), EventLoopError> {
    Game::run()
}
