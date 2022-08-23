pub struct Input {
    pub(crate) input_helper: winit_input_helper::WinitInputHelper,
    pub(crate) mouse_lock: bool,
    pub(crate) mouse_difference: (f32, f32),
}

pub type Key = winit::event::VirtualKeyCode;

impl Input {
    pub(crate) fn new() -> Self {
        Self {
            input_helper: winit_input_helper::WinitInputHelper::new(),
            mouse_lock: false,
            mouse_difference: (0.0, 0.0),
        }
    }

    pub(crate) fn update<T>(&mut self, event: &winit::event::Event<T>) {
        self.input_helper.update(event);
    }

    pub fn key_pressed(&self, key: Key) -> bool {
        self.input_helper.key_pressed(key)
    }

    pub fn key_released(&self, key: Key) -> bool {
        self.input_helper.key_released(key)
    }

    pub fn key_held(&self, key: Key) -> bool {
        self.input_helper.key_held(key)
    }

    pub fn window_resized(&self) -> Option<winit::dpi::PhysicalSize<u32>> {
        self.input_helper.window_resized()
    }

    pub fn mouse_delta(&mut self) -> (f32, f32) {
        let mut offset = self.input_helper.mouse_diff();
        if self.mouse_lock {
            offset = (self.mouse_difference.0 as f32 - offset.0, self.mouse_difference.1 as f32 - offset.1);
        }
        offset
    }

    pub fn mouse_pos(&self) -> Option<(f32, f32)> {
        self.input_helper.mouse()
    }

    pub fn mouse_lock(&mut self, lock: bool) {
        self.mouse_lock = lock;
    }

    pub fn quit(&self) -> bool {
        self.input_helper.quit()
    }
}