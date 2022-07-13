use heptagon::main_loop::*;
use heptagon::rendering::wgpu;
use heptagon::rendering::utils::*;

pub struct Game<'a> {
    renderer: Renderer<'a>,
    texture: Texture,
    camera: Camera,
    projection: Projection,
    cursor_locked: bool,
    yaw: f32,
    pitch: f32,
    font: Font,
}

impl<'a> Game<'a> {
    pub fn new(window: &Window, renderer: Renderer<'a>) -> Self {
        let texture = Texture::from_path(&renderer.device, &renderer.queue, "assets/images/rust.png", "happy-tree.png").unwrap();

        let camera = Camera::new(glam::Vec3::new(0.0, 0.0, 2.0), glam::Vec3::new(0.0, 0.0, 1.0));
        let mut font = Font::from_path("assets/fonts/Roboto-Regular.ttf", &renderer.device, renderer.config.format);

        let projection = Projection::new(renderer.config.width, renderer.config.height, 0.785398163, 0.1, 100.0);

        let texture = font.get_texture((512, 512), &renderer.device, &renderer.queue, "font texture");

        Self {
            renderer,
            texture,
            camera,
            projection,
            cursor_locked: false,
            yaw: -3.141592 / 2.0,
            pitch: 0.0,
            font,
        }
    }
}

impl<'a> Loop for Game<'a> {
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

        
        // ------- MOUSE -------
        let offset = input.mouse_delta();
        
        self.yaw += offset.0 * delta * mouse_speed;
        self.pitch -= offset.1 * delta * mouse_speed;
        
        let yaw: f32 = self.yaw;
        let pitch: f32 = self.pitch;
        self.camera.set_angles(yaw, pitch);
        // let camera_yaw = self.camera.get_yaw();
        // let camera_pitch = self.camera.get_pitch();
        // println!("Yaw: {yaw}, Calculated yaw: {camera_yaw}");
        // println!("Pitch: {pitch}, Calculated pitch: {camera_pitch}");

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

        if input.key_pressed(Key::C) {
            self.camera.set_direction(self.camera.get_position() - glam::Vec3::new(0.0, 0.0, -1.0));
            self.yaw = self.camera.get_yaw();
            self.pitch = self.camera.get_pitch();
        }
    }

    fn render(&mut self, window: &mut Window) {
        let scale = glam::Vec3::ONE;
        let translation = glam::Vec3::new(0.0, 0.0, 0.0);
        let rotation = glam::Quat::from_euler(glam::EulerRot::XYZ, 0.0, 0.0, 0.0);
        let model = Model::new(scale, translation, rotation);

        // self.renderer.render_texture(&self.texture, self.camera.get_view_mat(), 
        // self.projection.get_projection_mat(), model.get_model_mat());

        // let scale = glam::Vec3::ONE;
        // let translation = glam::Vec3::new(0.0, 0.0, -10.0);
        // let rotation = glam::Quat::from_euler(glam::EulerRot::XYZ, 0.0, 0.0, 0.0);
        // let model = Model::new(scale, translation, rotation);


        // self.renderer.render_texture(&self.texture, self.camera.get_view_mat(), 
        // self.projection.get_projection_mat(), model.get_model_mat());

        let encoder = self.renderer.make_render_bundle_encoder();
        let texture_bind_group = self.texture.get_bind_group(&self.renderer.device);
        let mvp_bind_group = Mat4Uniform::new(
            self.projection.get_projection_mat() * 
            self.camera.get_view_mat() * model.get_model_mat()).get_bind_group(&self.renderer.device);

        let render_bundle = self.renderer.make_render_bundle(encoder, 
            &texture_bind_group, &mvp_bind_group);

        self.renderer.run_render_bundles(&[render_bundle]);
    }
}