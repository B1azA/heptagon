pub struct Input {
    pub(crate) input_helper: winit_input_helper::WinitInputHelper,
    pub(crate) mouse_lock: bool,
    pub(crate) mouse_delta: (f32, f32),
    pub(crate) scroll_delta: f32,
}

use super::Key;

impl Input {
    /// Creates a new struct.
    pub(crate) fn new() -> Self {
        Self {
            input_helper: winit_input_helper::WinitInputHelper::new(),
            mouse_lock: false,
            mouse_delta: (0.0, 0.0),
            scroll_delta: 0.0,
        }
    }

    /// Updates itself with a winit event.
    pub(crate) fn update<T>(&mut self, event: &winit::event::Event<T>) {
        self.input_helper.update(event);
        if let winit::event::Event::DeviceEvent { device_id, event } = event {
            match event {
                winit::event::DeviceEvent::MouseMotion {
                    delta
                } => {
                    self.mouse_delta.0 += delta.0 as f32;
                    self.mouse_delta.1 += delta.1 as f32;
                }

                winit::event::DeviceEvent::MouseWheel {
                    delta
                } => {
                    if let winit::event::MouseScrollDelta::PixelDelta(position_delta) = delta {
                        self.scroll_delta = position_delta.x as f32;
                    }
                }

                _ => {}
            }
        }
    }

    /// This function should be called after every update function.
    pub(crate) fn updated(&mut self) {
        self.mouse_delta = (0.0, 0.0);
    }

    /// Returns true if a keyboard key was pressed.
    pub fn key_pressed(&self, key: Key) -> bool {
        self.input_helper.key_pressed(key)
    }

    /// Returns true if a keyboard key was released.
    pub fn key_released(&self, key: Key) -> bool {
        self.input_helper.key_released(key)
    }

    /// Returns true if a keyboard key was held.
    pub fn key_held(&self, key: Key) -> bool {
        self.input_helper.key_held(key)
    }

    /// Returns true if a keyboard key was released.
    pub fn window_resized(&self) -> Option<(u32, u32)> {
        if let Some(size) = self.input_helper.window_resized() {
            return Some((size.width, size.height));
        }

        return None;
    }

    /// Returns a mouse position delta relative to last update call.
    pub fn mouse_delta(&mut self) -> (f32, f32) {
        let delta = self.mouse_delta;
        // self.mouse_delta = (0.0, 0.0);
        delta
    }

    /// 
    pub fn scroll_delta(&self) -> f32 {
        self.input_helper.scroll_diff()
    }

    /// Returns the mouse position relative to a window position.
    /// Returns None if cursor is outside of a window.
    pub fn mouse_pos(&self) -> Option<(f32, f32)> {
        self.input_helper.mouse()
    }

    /// Locks the mouse in the centre of window. 
    pub fn set_mouse_lock(&mut self, lock: bool) {
        self.mouse_lock = lock;
    }

    /// Gets a state of the mouse lock;
    pub fn mouse_lock(&mut self) -> bool {
        self.mouse_lock
    }

    /// Returns true if the window will be closed.
    pub fn should_quit(&self) -> bool {
        self.input_helper.quit()
    }

    /// Returns a path to a file dropped to the window.
    /// Returns None if any file hasn't been dropped.
    pub fn dropped_file(&self) -> Option<std::path::PathBuf> {
        self.input_helper.dropped_file()
    }

    /// Returns true if a mouse button was pressed.
    pub fn mouse_pressed(&self, button: usize) -> bool {
        self.input_helper.mouse_pressed(button)
    }

    /// Returns true if a mouse button was released.
    pub fn mouse_released(&self, button: usize) -> bool {
        self.input_helper.mouse_released(button)
    }

    /// Returns true if a mouse button was held.
    pub fn mouse_held(&self, button: usize) -> bool {
        self.input_helper.mouse_held(button)
    }
}