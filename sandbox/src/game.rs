use heptagon::main_loop::*;
use heptagon::rendering::wgpu;
use heptagon::rendering::utils::{texture::Texture, camera::Camera };

pub struct Game {
    renderer2d: Renderer2D,
    texture: Texture,
    camera: Camera,
    cursor_locked: bool,
    last_mouse_pos: (f32, f32),
    new_mouse_pos: (f32, f32),
}

impl Game {
    pub fn new(window: &Window) -> Self {
        let renderer2d = Renderer2D::new(window);
        let texture = Texture::from_file(&renderer2d.device, &renderer2d.queue, "images/happy-tree.png", "happy-tree.png").unwrap();
        let mut camera = Camera {
            eye: (0.0, 0.0, 2.0).into(),
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
            camera,
            cursor_locked: false,
            last_mouse_pos: (0.0, 0.0),
            new_mouse_pos: (0.0, 0.0),
        }
    }
}

impl Loop for Game {
    fn init(&mut self, window: &mut Window) {
    }

    fn update(&mut self, window: &mut Window, delta: f64, input: &mut Input) {
        if input.key_held(VirtualKeyCode::Space) {
            println!("FPS: {:.2}", 1.0 / delta);
        }
        if let Some(size) = input.window_resized() {
            self.renderer2d.resize(size);
        }

        if input.key_held(VirtualKeyCode::W) {
            self.camera.eye += self.camera.forward().normalize() / 5.0;
        }
        if input.key_held(VirtualKeyCode::S) {
            self.camera.eye -= self.camera.forward().normalize() / 5.0;
        }
        if input.key_held(VirtualKeyCode::A) {
            self.camera.eye -= self.camera.right().normalize() / 5.0;
        }
        if input.key_held(VirtualKeyCode::D) {
            self.camera.eye += self.camera.right().normalize() / 5.0;
        }

        // ------- MOUSE -------
        
        let offset = input.mouse_delta();

        self.camera.target.x += offset.0 / 100.0;
        self.camera.target.y -= offset.1 / 100.0;
        
        if input.key_pressed(Key::L) {
            self.cursor_locked = !self.cursor_locked;
            input.mouse_lock(self.cursor_locked);

            if self.cursor_locked {
                println!("Cursor: locked");
            } else {
                println!("Cursor: unlocked");
            }
        }

        if input.key_pressed(Key::G) {
            window.set_cursor_grab(true).unwrap();
            println!("Cursor: grabbed");
        }

        if input.key_pressed(Key::U) {
            window.set_cursor_grab(false).unwrap();
            println!("Cursor: ungrabbed");
        }
    }

    fn render(&mut self, window: &mut Window) {  
        self.renderer2d.render_texture(&self.texture, &self.camera);
    }
}