use heptagon::main_loop::*;

pub struct Game {
}

impl Game {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Loop for Game {
    fn init(&mut self, window: &mut Window) {
        window.set_title("Heptagon");
        let size = window.inner_size();
        let width = size.width;
    }

    fn update(&mut self, window: &mut Window, delta: f64, input: &WinitInputHelper) {
        if input.key_pressed(VirtualKeyCode::Space) {
            println!("FPS: {:.2}", 1.0 / delta);
        }
    }

    fn render(&mut self, window: &mut Window, render_package: &mut RenderPackage) {
        render_package.renderer2d.render();
    }
}