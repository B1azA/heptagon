use crate::rendering::render_pipeline::*;
use crate::rendering::*;

pub struct Bundle {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
}

impl Bundle {
    pub fn surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }

    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    pub fn new_custom(
        surface: wgpu::Surface,
        device: wgpu::Device,
        queue: wgpu::Queue,
        config: wgpu::SurfaceConfiguration,
    ) -> Self {
        async_std::task::block_on(Self::async_new_custom(surface, device, queue, config))
    }

    async fn async_new_custom(
        surface: wgpu::Surface,
        device: wgpu::Device,
        queue: wgpu::Queue,
        config: wgpu::SurfaceConfiguration,
    ) -> Self {
        surface.configure(&device, &config);

        let size = (config.width, config.height).into();

        Self {
            surface,
            device,
            queue,
            config,
            size,
        }
    }

    pub fn new(window: &winit::window::Window) -> Self {
        async_std::task::block_on(Self::async_new(window))
    }

    async fn async_new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        Self::async_new_custom(surface, device, queue, config).await
    }

    pub fn surface_texture(&self) -> wgpu::SurfaceTexture {
        self.surface.get_current_texture().unwrap()
    }

    pub fn encoder(&self) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            })
    }

    pub fn preffered_format(&self) -> wgpu::TextureFormat {
        self.config().format
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.size = size;
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn texture_pipeline(&self) -> RenderPipeline {
        let texture_bind_group_layout = super::Texture::bind_group_layout(&self.device);
        let mvp_bind_group_layout = super::Uniform::<glam::Mat4>::bind_group_layout(&self.device);

        let texture_pipeline = RenderPipeline::new(
            &self.device,
            include_str!("../shaders/shader.wgsl"),
            &[&texture_bind_group_layout, &mvp_bind_group_layout],
            &[Vertices::<VertexTex>::vertex_buffer_layout()],
            self.config.format,
            true,
        );

        texture_pipeline
    }

    pub fn texture_pipeline_instanced(&self) -> RenderPipeline {
        let texture_bind_group_layout = super::Texture::bind_group_layout(&self.device);
        let mvp_bind_group_layout = super::Uniform::<glam::Mat4>::bind_group_layout(&self.device);

        let texture_pipeline = RenderPipeline::new(
            &self.device,
            include_str!("../shaders/instanced.wgsl"),
            &[&texture_bind_group_layout, &mvp_bind_group_layout],
            &[
                Vertices::<VertexTex>::vertex_buffer_layout(),
                Instance::vertex_buffer_layout(),
            ],
            self.config.format,
            true,
        );

        texture_pipeline
    }

    pub fn text_pipeline(&self) -> RenderPipeline {
        let texture_bind_group_layout = super::Texture::bind_group_layout(&self.device);
        let mvp_bind_group_layout = super::Uniform::<glam::Mat4>::bind_group_layout(&self.device);
        let color_bind_group_layout = super::Uniform::<glam::Vec4>::bind_group_layout(&self.device);

        let text_pipeline = RenderPipeline::new(
            &self.device,
            include_str!("../shaders/text.wgsl"),
            &[
                &texture_bind_group_layout,
                &mvp_bind_group_layout,
                &color_bind_group_layout,
            ],
            &[Vertices::<VertexTex>::vertex_buffer_layout()],
            self.config.format,
            true,
        );

        text_pipeline
    }
}
