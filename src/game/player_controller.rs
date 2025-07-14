use winit::{event::ElementState, event_loop::ActiveEventLoop, keyboard::KeyCode};

#[derive(Default)]
pub struct PlayerController {
    pub is_w_pressed: bool,
    pub is_s_pressed: bool,
    pub is_a_pressed: bool,
    pub is_d_pressed: bool,
    pub is_space_pressed: bool,
    pub delta_mouse_pos: Option<(f32, f32)>,
}

impl PlayerController {
    pub fn handle_key_held(
        &mut self,
        key: KeyCode,
        state: ElementState,
        event_loop: &ActiveEventLoop,
    ) -> bool {
        match key {
            KeyCode::Escape => {
                event_loop.exit();
                false
            }
            KeyCode::KeyW => {
                self.is_w_pressed = state.is_pressed();
                true
            }
            KeyCode::KeyS => {
                self.is_s_pressed = state.is_pressed();
                true
            }
            KeyCode::KeyD => {
                self.is_d_pressed = state.is_pressed();
                true
            }
            KeyCode::KeyA => {
                self.is_a_pressed = state.is_pressed();
                true
            }
            KeyCode::Space => {
                self.is_space_pressed = state.is_pressed();
                true
            }
            _ => false,
        }
    }

    pub fn handle_mouse(&mut self, delta: (f64, f64)) {
        let dx = delta.0 as f32;
        let dy = delta.1 as f32;
        self.delta_mouse_pos = Some((dx, dy));
    }
}
