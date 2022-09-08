use crate::rendering::render_pipeline::*;
use crate::rendering::*;

/// Struct to store multiple structs needed for rendering.
pub struct Bundle {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
}

impl Bundle {
    /// The default depth format.
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    /// Returns its surface.
    pub fn surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    /// Returns its device.
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    /// Returns its queue.
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    /// Returns its config.
    pub fn config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }

    /// Returns its size.
    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    /// Create a new Bundle with custom properties.
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

    /// Create a new Bundle.
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

    /// Returns its surface_texture. 
    pub fn surface_texture(&self) -> wgpu::SurfaceTexture {
        self.surface.get_current_texture().unwrap()
    }

    /// Returns its surface_view.
    pub fn surface_view(&self) -> wgpu::TextureView {
        self.surface_texture().texture
        .create_view(&wgpu::TextureViewDescriptor::default())
    }

    /// Creates a new encoder.
    pub fn encoder(&self) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            })
    }

    /// Returns preffered format by your PC.
    pub fn preffered_format(&self) -> wgpu::TextureFormat {
        self.config().format
    }

    /// Resizes its surface.
    /// Should be called when window has been resized.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Creates a pipeline for textures.
    pub fn texture_pipeline(&self) -> RenderPipeline {
        let texture_bind_group_layout = super::Texture::bind_group_layout(self);
        let mvp_bind_group_layout = super::Uniform::<glam::Mat4>::bind_group_layout(self);

        let texture_pipeline = RenderPipeline::new(
            &self.device,
            include_str!("../shaders/shader.wgsl"),
            &[&texture_bind_group_layout, &mvp_bind_group_layout],
            &[Vertices::<TextureVertex>::vertex_buffer_layout()],
            self.config.format,
            Some(wgpu::DepthStencilState {
                format: Self::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
        );

        texture_pipeline
    }

    /// Creates a pipeline for instanced textures.
    pub fn texture_pipeline_instanced(&self) -> RenderPipeline {
        let texture_bind_group_layout = super::Texture::bind_group_layout(self);
        let mvp_bind_group_layout = super::Uniform::<glam::Mat4>::bind_group_layout(self);

        let texture_pipeline = RenderPipeline::new(
            &self.device,
            include_str!("../shaders/instanced.wgsl"),
            &[&texture_bind_group_layout, &mvp_bind_group_layout],
            &[
                Vertices::<TextureVertex>::vertex_buffer_layout(),
                Instance::vertex_buffer_layout(),
            ],
            self.config.format,
            Some(wgpu::DepthStencilState {
                format: Self::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
        );

        texture_pipeline
    }

    /// Creates a pipeline for text.
    pub fn text_pipeline(&self) -> RenderPipeline {
        let texture_bind_group_layout = super::Texture::bind_group_layout(self);
        let mvp_bind_group_layout = super::Uniform::<glam::Mat4>::bind_group_layout(self);
        let color_bind_group_layout = super::Uniform::<glam::Vec4>::bind_group_layout(self);

        let text_pipeline = RenderPipeline::new(
            &self.device,
            include_str!("../shaders/text.wgsl"),
            &[
                &texture_bind_group_layout,
                &mvp_bind_group_layout,
                &color_bind_group_layout,
            ],
            &[Vertices::<TextureVertex>::vertex_buffer_layout()],
            self.config.format,
            Some(wgpu::DepthStencilState {
                format: Self::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
        );

        text_pipeline
    }
}
