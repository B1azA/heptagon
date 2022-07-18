use heptagon::main_loop::*;
use heptagon::rendering::*;

pub struct Game {
    renderer: Renderer,
    texture: Texture,
    texture2: Texture,
    camera: Camera,
    projection: Projection,
    cursor_locked: bool,
    yaw: f32,
    pitch: f32,
    font: Font,
}

impl Game {
    pub fn new(window: &Window, renderer: Renderer) -> Self {
        let texture2 = Texture::from_path(renderer.device(), renderer.queue(),
            "assets/images/rust.png", "happy-tree.png").unwrap();

        let camera = Camera::new(glam::Vec3::new(0.0, 0.0, 2.0), glam::Vec3::new(0.0, 0.0, 1.0));

        let projection = Projection::new(renderer.config().width, renderer.config().height,
            0.785398163, 0.1, 100.0);
        
        let font = Font::from_path("assets/fonts/Roboto-Regular.ttf");

        let texture = font.create_texture(renderer.device(), renderer.queue());

        Self {
            renderer,
            texture,
            texture2,
            camera,
            projection,
            cursor_locked: false,
            yaw: -3.141592 / 2.0,
            pitch: 0.0,
            font,
        }
    }
}

impl Loop for Game {
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
            let shift = self.camera.forward().normalize() * delta * move_speed;
            self.camera.set_position(self.camera.position() + shift);
        }
        if input.key_held(Key::S) {
            let shift = self.camera.forward().normalize() * delta * move_speed;
            self.camera.set_position(self.camera.position() - shift);
        }
        if input.key_held(Key::A) {
            let shift = self.camera.right().normalize() * delta * move_speed;
            self.camera.set_position(self.camera.position() - shift);
        }
        if input.key_held(Key::D) {
            let shift = self.camera.right().normalize() * delta * move_speed;
            self.camera.set_position(self.camera.position() + shift);
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
            self.camera.set_direction(self.camera.position() - glam::Vec3::new(0.0, 0.0, -1.0));
            self.yaw = self.camera.yaw();
            self.pitch = self.camera.pitch();
        }
    }
    
    fn render(&mut self, window: &mut Window) {
        let scale = glam::Vec3::ONE;
        let translation = glam::Vec3::new(0.0, 0.0, 0.0);
        let rotation = glam::Quat::from_euler(glam::EulerRot::XYZ, 0.0, 0.0, 0.0);
        let model = Model::new(scale, translation, rotation);

        let mvp_uniform = Uniform::new(
            self.projection.projection_mat() * 
            self.camera.view_mat() * model.model_mat());
        let mvp_bind_group = mvp_uniform.bind_group(&self.renderer.device());

        let texture_bind_group = self.texture.bind_group(&self.renderer.device());

        let mut render_queue = RenderQueue::begin(
            &self.renderer.device(),
            &self.renderer.texture_pipeline,
            &self.renderer.text_pipeline,
        );

        // render_queue.render_texture(
        //     self.renderer.vertex_buffer.slice(..),
        //     self.renderer.index_buffer.slice(..),
        //     self.renderer.indices_count as u32,
        //     &texture_bind_group,
        //     &mvp_bind_group,
        // );

        let color_uniform = Uniform::new(glam::Vec4::new(1.0, 0.0, 0.0, 1.0));
        let color_bind_group = color_uniform.bind_group(&self.renderer.device());

        render_queue.render_text(
            self.renderer.vertex_buffer.slice(..),
            self.renderer.index_buffer.slice(..),
            self.renderer.indices_count as u32,
            &texture_bind_group,
            &mvp_bind_group,
            &color_bind_group,
        );

        let texture_bind_group2 = self.texture2.bind_group(&self.renderer.device());

        render_queue.render_texture(
            self.renderer.vertex_buffer.slice(..),
            self.renderer.index_buffer.slice(..),
            self.renderer.indices_count as u32,
            &texture_bind_group2,
            &mvp_bind_group,
        );
        
        let bundle = render_queue.finish();

        self.renderer.run_render_bundles(&[bundle]);
    }
}