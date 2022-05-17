use heptagon::main_loop::*;
use heptagon::rendering::wgpu;

pub struct Game {
    renderer2d: Renderer2D,
}

impl Game {
    pub fn new(window: &Window) -> Self {
        Self {
            renderer2d: Renderer2D::new(window),
        }
    }
}

impl Loop for Game {
    fn init(&mut self, window: &mut Window) {
    }

    fn update(&mut self, window: &mut Window, delta: f64, input: &WinitInputHelper) {
        if input.key_pressed(VirtualKeyCode::Space) {
            println!("FPS: {:.2}", 1.0 / delta);
        }
        if let Some(size) = input.window_resized() {
            self.renderer2d.resize(size);
        }
    }

    fn render(&mut self, window: &mut Window) {
        self.renderer2d.render();
    }
}