use heptagon::main_loop::*;
use heptagon::rendering::wgpu;
use heptagon::rendering::utils::{texture::Texture, camera::Camera };

pub struct Game {
    renderer2d: Renderer2D,
    texture: Texture,
    camera: Camera,
}

impl Game {
    pub fn new(window: &Window) -> Self {
        let renderer2d = Renderer2D::new(window);
        let texture = Texture::from_file(&renderer2d.device, &renderer2d.queue, "images/happy-tree.png", "happy-tree.png").unwrap();
        let camera = Camera {
            eye: (0.0, 1.0, 2.0).into(),
            target: glam::Vec3::new(0.0, 0.0, 0.0),
            up: glam::Vec3::new(0.0, 1.0, 0.0),
            aspect: renderer2d.config.width as f32 / renderer2d.config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
            speed: 0.1,
        };
        Self {
            renderer2d,
            texture,
            camera
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

        if input.key_held(VirtualKeyCode::Left) {
            self.camera.shift(glam::Vec3::new(-1.0, 0.0, 0.0));
        }

        if input.key_held(VirtualKeyCode::Right) {
            self.camera.shift(glam::Vec3::new(1.0, 0.0, 0.0));
        }

        if input.key_held(VirtualKeyCode::Up) {
            self.camera.shift(glam::Vec3::new(0.0, 0.0, -1.0));
        }

        if input.key_held(VirtualKeyCode::Down) {
            self.camera.shift(glam::Vec3::new(0.0, 0.0, 1.0));
        }
    }

    fn render(&mut self, window: &mut Window) {  
        self.renderer2d.render_texture(&self.texture, &self.camera);
    }
}