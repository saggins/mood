mod application;
mod renderer;

use application::AppState;
use wgpu::wgc::instance::RequestAdapterOptions;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::{application::ApplicationHandler, window::Window};

pub struct Game;

impl Game {
    pub fn run() -> Result<(), EventLoopError> {
        env_logger::init();
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.set_control_flow(ControlFlow::Wait);
        let mut app = AppState::default();

        event_loop.run_app(&mut app)
    }
}
