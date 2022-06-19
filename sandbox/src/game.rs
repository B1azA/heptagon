use heptagon::main_loop::*;
use heptagon::rendering::wgpu;
use heptagon::rendering::utils::{texture::Texture, camera::Camera, text::Font, projection::Projection, };

pub struct Game {
    renderer: Renderer,
    texture: Texture,
    camera: Camera,
    projection: Projection,
    cursor_locked: bool,
    yaw: f32,
    pitch: f32,
}

impl Game {
    pub fn new(window: &Window, renderer: Renderer) -> Self {
        let renderer = renderer;
        let texture = Texture::from_path(&renderer.device, &renderer.queue, "assets/images/rust.png", "happy-tree.png").unwrap();

        let camera = Camera::new(glam::Vec3::new(0.0, 0.0, 2.0), glam::Vec3::new(0.0, 0.0, 1.0), glam::Vec3::Y);

        // let camera = Camera2::new(glam::Vec3::new(0.0, 0.0, 2.0), -3.141592 / 2.0, 0.0);

        let projection = Projection::new(renderer.config.width, renderer.config.height, 0.785398163, 0.1, 100.0);

        let font = Font::from_path("assets/fonts/Roboto-Regular.ttf");
        let texture = Texture::from_image(&renderer.device, &renderer.queue,
            &font.get_image("This is RustType rendered into a png!", 32.0, (150, 0, 0)), "text_texture").unwrap();

        Self {
            renderer,
            texture,
            camera,
            projection,
            cursor_locked: false,
            yaw: -3.141592 / 2.0,
            pitch: 0.0,
        }
    }
}

impl Loop for Game {
    fn init(&mut self, window: &mut Window) {
    }

    fn update(&mut self, window: &mut Window, delta: f32, input: &mut Input) {

        let move_speed = 4.0;
        let mouse_speed = 0.5;

        if input.key_held(Key::Space) {
            println!("UPS: {:.2}", 1.0 / delta);
            println!("Delta: {}", delta);
        }
        if let Some(size) = input.window_resized() {
            self.renderer.resize(size);
            self.projection.resize(size.width, size.height)
        }

        if input.key_held(Key::W) {
            let shift = self.camera.get_forward().normalize() * delta * move_speed;
            self.camera.set_position(self.camera.get_position() + shift);
        }
        if input.key_held(Key::S) {
            let shift = self.camera.get_forward().normalize() * delta * move_speed;
            self.camera.set_position(self.camera.get_position() - shift);
        }
        if input.key_held(Key::A) {
            let shift = self.camera.get_right().normalize() * delta * move_speed;
            self.camera.set_position(self.camera.get_position() - shift);
        }
        if input.key_held(Key::D) {
            let shift = self.camera.get_right().normalize() * delta * move_speed;
            self.camera.set_position(self.camera.get_position() + shift);
        }

        if input.key_pressed(Key::C) {
            self.camera.set_target(glam::Vec3::new(0.0, 0.0, 0.0));
        }

        // ------- MOUSE -------
        let offset = input.mouse_delta();
        
        self.yaw += offset.0 * delta * mouse_speed;
        self.pitch -= offset.1 * delta * mouse_speed;

        let yaw: f32 = self.yaw;
        let pitch: f32 = self.pitch;
        self.camera.set_angles(yaw, pitch);

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
        let scale = glam::Vec3::ONE;
        let translation = glam::Vec3::new(0.0, 0.0, 0.0);
        let quat = glam::Quat::from_euler(glam::EulerRot::XYZ, 0.0, 0.0, 0.0);
        let model = glam::Mat4::from_scale_rotation_translation(scale, quat, translation);

        self.renderer.render_texture(&self.texture, self.camera.get_view_mat(), self.projection.get_projection_mat(), model);
    }
}