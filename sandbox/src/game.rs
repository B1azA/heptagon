use heptagon::main_loop::*;
use heptagon::rendering::*;
use wgpu::util::DeviceExt;

pub struct Game {
    bundle: Bundle,
    texture_pipeline: RenderPipeline,
    text_pipeline: RenderPipeline,
    texture_pipeline_instanced: RenderPipeline,
    mesh: MeshBuffer,
    texture: Texture,
    texture2: Texture,
    camera: Camera,
    projection: Projection,
    cursor_locked: bool,
    yaw: f32,
    pitch: f32,
    font: Font,
    instances: Instances<Instance>,
    #[allow(dead_code)]
    instance_buffer: wgpu::Buffer,
    depth_texture: Texture,
}

impl Game {
    pub fn new(window: &Window, bundle: Bundle) -> Self {
        let texture_pipeline = bundle.texture_pipeline();
        let text_pipeline = bundle.text_pipeline();
        let texture_pipeline_instanced = bundle.texture_pipeline_instanced();

        let vertices = Vertices::new(vec![
            TextureVertex {
                position: glam::vec3(-0.5, 0.5, 0.0),
                tex_coords: glam::vec2(0.0, 0.0),
            }, // A
            TextureVertex {
                position: glam::vec3(-0.5, -0.5, 0.0),
                tex_coords: glam::vec2(0.0, 1.0),
            }, // B
            TextureVertex {
                position: glam::vec3(0.5, -0.5, 0.0),
                tex_coords: glam::vec2(1.0, 1.0),
            }, // C
            TextureVertex {
                position: glam::vec3(0.5, 0.5, 0.0),
                tex_coords: glam::vec2(1.0, 0.0),
            }, // D
        ]);

        let indices = Indices::<u16>::new(vec![0, 1, 2, 2, 3, 0]);

        let mesh = Mesh::new(vertices, indices).mesh_buffer(&bundle.device());

        let texture2 = Texture::from_path(
            bundle.device(),
            bundle.queue(),
            "assets/images/happy-tree.png",
            "happy-tree.png",
        )
        .unwrap();

        let camera = Camera::new(
            glam::Vec3::new(0.0, 0.0, 2.0),
            glam::Vec3::new(0.0, 0.0, 1.0),
        );

        let projection = Projection::new(
            bundle.config().width,
            bundle.config().height,
            0.785398163,
            0.1,
            100.0,
        );

        let font = Font::from_path("assets/fonts/Roboto-Regular.ttf");

        let texture = font.glyph_texture(bundle.device(), bundle.queue(), 'a', 100.0);

        let num_instances_per_row = 10;

        let instance_displacement: glam::Vec3 = glam::vec3(
            num_instances_per_row as f32 * 0.5,
            0.0,
            num_instances_per_row as f32 * 0.5,
        );

        let instances = (0..num_instances_per_row)
            .flat_map(|z| {
                (0..num_instances_per_row).map(move |x| {
                    let position = glam::vec3(x as f32, 0.0, z as f32) - instance_displacement;

                    let rotation = if position == glam::Vec3::ZERO {
                        // this is needed so an object at (0, 0, 0) won't get scaled to zero
                        // as Quaternions can effect scale if they're not created correctly
                        glam::Quat::from_axis_angle(
                            glam::Vec3::Z,
                            0.0,
                        )
                    } else {
                        glam::Quat::from_axis_angle(position.normalize(), std::f32::consts::PI / 4.0)
                    };

                    Instance::new(Model::new(glam::Vec3::ONE, position, rotation).model_mat())
                })
            })
            .collect::<Vec<_>>();

        let instances = Instances::new(instances);

        let instance_buffer = bundle.device().create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: instances.to_bytes(),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let depth_texture = Texture::depth_texture(bundle.device(), bundle.config(),
            Bundle::DEPTH_FORMAT, "depth_texture");

        Self {
            bundle,
            texture_pipeline,
            text_pipeline,
            texture_pipeline_instanced,
            mesh,
            texture,
            texture2,
            camera,
            projection,
            cursor_locked: false,
            yaw: -3.141592 / 2.0,
            pitch: 0.0,
            font,
            instances,
            instance_buffer,
            depth_texture,
        }
    }
}

impl Loop for Game {
    fn update(&mut self, window: &mut Window, delta: f32, input: &mut Input) {

        if input.key_held(Key::Space) {
            println!("UPS: {:.2}", 1.0 / delta);
            println!("Delta: {}", delta);
        }
        if let Some(size) = input.window_resized() {
            self.bundle.resize(size);
            self.projection.resize(size.width, size.height);
            self.depth_texture = Texture::depth_texture(self.bundle.device(), self.bundle.config(),
            Bundle::DEPTH_FORMAT, "depth_texture");
        }

        // -------- MOVEMENT --------

        let move_speed = 5.5;
        let mouse_speed = 0.5;

        let mut forward = 0.0;
        let mut right = 0.0;
        let mut up = 0.0;

        if input.key_held(Key::W) {
            forward += 1.0;
        }
        if input.key_held(Key::S) {
            forward += -1.0;
        }
        if input.key_held(Key::A) {
            right += -1.0;
        }
        if input.key_held(Key::D) {
            right += 1.0;
        }
        if input.key_held(Key::Q) {
            up += 1.0;
        }
        if input.key_held(Key::E) {
            up += -1.0;
        }

        let offset = input.mouse_delta();

        self.yaw += offset.0 * delta * mouse_speed;
        self.pitch -= offset.1 * delta * mouse_speed;

        self.camera.set_angles_mut(&mut self.yaw, &mut self.pitch);
        self.camera.shift(
            forward * delta * move_speed, 
            right * delta * move_speed, 
            up * delta * move_speed);

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
            self.camera.set_target(glam::vec3(0.0, 0.0, 0.0));

            self.yaw = self.camera.yaw();
            self.pitch = self.camera.pitch();
            println!("{}, {}", self.yaw, self.pitch);
        }
    }

    fn render(&mut self, window: &mut Window) {
        let scale = glam::Vec3::ONE;
        let translation = glam::Vec3::new(0.0, 0.0, 0.0);
        let rotation = glam::Quat::from_euler(glam::EulerRot::XYZ, 0.0, 0.0, 0.0);
        let model = Model::new(scale, translation, rotation);

        let mvp_uniform = Uniform::new(
            self.projection.projection_mat() * self.camera.view_mat() * model.model_mat(),
        );
        let mvp_bind_group = mvp_uniform.bind_group(&self.bundle.device());

        let texture_bind_group = self.texture.bind_group(&self.bundle.device());

        let output = self.bundle.surface_texture();
        let mut encoder = self.bundle.encoder();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let color_uniform = Uniform::new(glam::Vec4::new(1.0, 1.0, 1.0, 1.0));
        let color_bind_group = color_uniform.bind_group(&self.bundle.device());

        let texture_bind_group2 = self.texture2.bind_group(&self.bundle.device());

        let vp_uniform = Uniform::new(self.projection.projection_mat() * self.camera.view_mat());
        let vp_bind_group = vp_uniform.bind_group(&self.bundle.device());

        let mut render_pass = RenderPass::begin_with_depth(
            &mut encoder,
            &view,
            &self.texture_pipeline,
            &self.text_pipeline,
            &self.texture_pipeline_instanced,
            [0.1, 0.2, 0.3, 0.0],
            self.depth_texture.view()
        );

        // render_pass.render_text(
        //     self.mesh.vertex_buffer_slice(),
        //     self.mesh.index_buffer_slice(),
        //     0..self.mesh.index_count(),
        //     &texture_bind_group,
        //     &mvp_bind_group,
        //     &color_bind_group,
        // );

        // render_pass.render_texture(
        //     self.mesh.vertex_buffer_slice(),
        //     self.mesh.index_buffer_slice(),
        //     0..self.mesh.index_count(),
        //     &texture_bind_group2,
        //     &mvp_bind_group,
        // );

        render_pass.render_texture_instanced(
            self.mesh.vertex_buffer_slice(),
            self.mesh.index_buffer_slice(),
            0..self.mesh.index_count(),
            &texture_bind_group2,
            &vp_bind_group,
            self.instance_buffer.slice(..),
            0..self.instances.len() as u32
        );

        render_pass.end();

        self.bundle.queue().submit(Some(encoder.finish()));
        output.present();
    }
}