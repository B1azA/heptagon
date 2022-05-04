use std::time::{Instant};

use winit::dpi::{Size, PhysicalSize};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{ WindowBuilder},
};

pub use winit::window::Window;

pub struct MainLoop {
    event_loop: EventLoop<()>,
    pub window: Window,
}

impl MainLoop {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Hello")
            .with_inner_size(Size::Physical(PhysicalSize::new(800, 600)))
            .build(&event_loop)
            .unwrap();

        MainLoop {
            event_loop,
            window,
        }
    }

    pub fn run(self, loops: impl Loop + std::marker::Send + 'static) {
        async_std::task::block_on(self.runrun(loops));
    }

    async fn runrun(mut self, mut loops: impl Loop + std::marker::Send + 'static) {
        loops.init(&mut self.window);

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&self.window) };
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
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        ).await.unwrap();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: self.window.inner_size().width,
            height: self.window.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);


        let mut i = 0;
        let mut now = Instant::now();
        let mut last = Instant::now();
        let mut delta = 0.0;

        self.event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == self.window.id() => match event {
                WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit },
                WindowEvent::Resized(physical_size) => {
                    // size = (physical_size.width, physical_size.height)
                },
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {},
                    // set new size
                _ => {}
            },
            Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                now = Instant::now();
                delta = now.duration_since(last).as_millis() as f64 / 1000.0;
                loops.update(&mut self.window, delta);
                last = now;

                loops.render(&mut self.window);

                let output = surface.get_current_texture().unwrap();
                let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
                });

                i += 1;
                if i > 255 {
                    i = 0;
                }

                {
                    let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: i as f64 / 255.0,
                                    g: i as f64 / 255.0,
                                    b: i as f64 / 255.0,
                                    a: 1.0,
                                }),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                }

                queue.submit(std::iter::once(encoder.finish()));
                output.present();
            },
            Event::MainEventsCleared => {
                self.window.request_redraw();
            }

            _ => {}
        });
    }
}

pub trait Loop {
    fn init(&mut self, window: &mut Window);
    fn update(&mut self, window: &mut Window, delta: f64);
    fn render(&mut self, window: &mut Window);
}