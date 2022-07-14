use wgpu::util::DeviceExt;
use image::GenericImageView;
use crate::rendering::utils::*;

pub struct Renderer<'a> {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    _vertices: Vertices<'a, VertexTex>,
    pub indices: Indices<'a, u16>,
}

impl<'a> Renderer<'a> {
    pub fn custom_new(surface: wgpu::Surface, device: wgpu::Device, queue: wgpu::Queue, config: wgpu::SurfaceConfiguration) -> Self {
        async_std::task::block_on(Self::custom_newnew(surface, device, queue, config))
    }

    async fn custom_newnew(surface: wgpu::Surface, device: wgpu::Device, queue: wgpu::Queue, config: wgpu::SurfaceConfiguration) -> Renderer<'a> {

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
        
        let vertices = Vertices::new(
            &[
                VertexTex { position: [-0.5, 0.5, 0.0], tex_coords: [0.0, 0.0], }, // A
                VertexTex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 1.0], }, // B
                VertexTex { position: [0.5, -0.5, 0.0], tex_coords: [1.0, 1.0], }, // C
                VertexTex { position: [0.5, 0.5, 0.0], tex_coords: [1.0, 0.0], }, // D
            ]
        );

        let vertex_buffer = vertices.to_vertex_buffer(&device);

        let index_buffer = indices.to_index_buffer(&device);

        let texture_bind_group_layout = Texture::get_bind_group_layout(&device);
 
        let mvp_bind_group_layout = Mat4Uniform::get_bind_group_layout(&device);
        
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &texture_bind_group_layout,
                &mvp_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    Vertices::<VertexTex>::layout(),
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let size = (config.width, config.height).into();

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            _vertices: vertices,
            indices,
        }
    }

    pub fn new(window: &winit::window::Window) -> Self {
        async_std::task::block_on(Self::newnew(window))
    }
    
    async fn newnew(window: &winit::window::Window) -> Renderer<'a> {
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

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.size = size;
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}