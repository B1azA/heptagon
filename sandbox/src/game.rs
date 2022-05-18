use heptagon::main_loop::*;
use heptagon::rendering::wgpu;
use heptagon::rendering::utils::texture::Texture;

pub struct Game {
    renderer2d: Renderer2D,
    texture: Texture,
}

impl Game {
    pub fn new(window: &Window) -> Self {
        let renderer2d = Renderer2D::new(window);
        let texture = Texture::from_file(&renderer2d.device, &renderer2d.queue, "images/happy-tree.png", "happy-tree.png").unwrap();
        Self {
            renderer2d,
            texture,
        }
    }
}

impl Loop for Game {
    fn init(&mut self, window: &mut Window) {
    }

    fn update(&mut self, window: &mut Window, delta: f64, input: &WinitInputHelper) {
        if input.key_held(VirtualKeyCode::Space) {
            println!("FPS: {:.2}", 1.0 / delta);
        }
        if let Some(size) = input.window_resized() {
            self.renderer2d.resize(size);
        }
    }

    fn render(&mut self, window: &mut Window) {  
        self.renderer2d.render_texture(&self.texture);
    }
}