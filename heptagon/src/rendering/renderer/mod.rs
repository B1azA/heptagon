use crate::rendering::*;
use crate::rendering::render_pipeline::*;

pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    pub texture_pipeline: RenderPipeline,
    pub text_pipeline: RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub indices_count: usize,
}

impl Renderer {
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

    pub fn custom_new(surface: wgpu::Surface, device: wgpu::Device, queue: wgpu::Queue, config: wgpu::SurfaceConfiguration) -> Self {
        async_std::task::block_on(Self::custom_newnew(surface, device, queue, config))
    }

    async fn custom_newnew(surface: wgpu::Surface, device: wgpu::Device, queue: wgpu::Queue,
        config: wgpu::SurfaceConfiguration) -> Renderer {

        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/shader.wgsl").into()),
        });

        let indices = Indices::<u16>::new(
            &[
                0, 1, 2,
                2, 3, 0,
            ]
        );

        let indices_count = indices.len();
        let vertices = Vertices::new(
            &[
                VertexTex { position: [-0.5, 0.5, 0.0], tex_coords: [0.0, 0.0], }, // A
                VertexTex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 1.0], }, // B
                VertexTex { position: [0.5, -0.5, 0.0], tex_coords: [1.0, 1.0], }, // C
                VertexTex { position: [0.5, 0.5, 0.0], tex_coords: [1.0, 0.0], }, // D
            ]
        );

        let vertex_buffer = vertices.vertex_buffer(&device);

        let index_buffer = indices.index_buffer(&device);

        let texture_bind_group_layout = super::Texture::bind_group_layout(&device);
 
        let mvp_bind_group_layout = super::Uniform::<glam::Mat4>::bind_group_layout(&device);

        let texture_pipeline = super::render_pipeline::RenderPipeline::new(
            &device,
            include_str!("../../shaders/shader.wgsl"),
            &[
                &texture_bind_group_layout,
                &mvp_bind_group_layout,
            ],
            &[vertices.buffer_layout()],
            config.format,
        );

        let color_bind_group_layout = super::Uniform::<glam::Vec4>::bind_group_layout(&device);

        let text_pipeline = RenderPipeline::new(
            &device,
            include_str!("../../shaders/text.wgsl"),
            &[
                &texture_bind_group_layout,
                &mvp_bind_group_layout,
                &color_bind_group_layout,
            ],
            &[vertices.buffer_layout(), wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: wgpu::VertexFormat::Float32x4,
                    },
                ]
            }],
            config.format,
        );

        let size = (config.width, config.height).into();

        Self {
            surface,
            device,
            queue,
            config,
            size,
            texture_pipeline,
            text_pipeline,
            vertex_buffer,
            index_buffer,
            indices_count,
        }
    }

    pub fn new(window: &winit::window::Window) -> Self {
        async_std::task::block_on(Self::newnew(window))
    }
    
    async fn newnew(window: &winit::window::Window) -> Renderer {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
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
        ).await.unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        Self::custom_new(surface, device, queue, config)
    }

    pub fn run_render_bundles(&self, render_bundles: &[wgpu::RenderBundle]) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(
                                wgpu::Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 0.0,
                                }
                            ),
                            store: true,
                        }
                    })
                ],
                depth_stencil_attachment: None,
            });

            render_pass.execute_bundles(render_bundles.iter());
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();
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
}