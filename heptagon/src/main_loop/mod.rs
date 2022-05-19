use std::time::{Instant};

use winit::dpi::{Size, PhysicalSize};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{ WindowBuilder},
};

pub use winit::window::Window;
pub use winit;
pub use winit::event::VirtualKeyCode;
pub use crate::rendering::renderer_2d::*;
pub use winit_input_helper::WinitInputHelper;
pub use glam;

pub struct MainLoop {
    event_loop: EventLoop<()>,
    pub window: Window,
    input: WinitInputHelper,
}

impl MainLoop {
    pub fn new(window_title: &str) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(window_title)
            .with_inner_size(Size::Physical(PhysicalSize::new(800, 600)))
            .build(&event_loop)
            .unwrap();

        MainLoop {
            event_loop,
            window,
            input: WinitInputHelper::new(),
        }
    }

    pub fn run(self, loops: impl Loop + std::marker::Send + 'static) {
        env_logger::init();
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

        let mut now = Instant::now();
        let mut last = Instant::now();
        let mut delta = 0.0;

        self.event_loop.run(move |event, _, control_flow| {
            self.input.update(&event);
            match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == self.window.id() => match event {
                WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit },
                WindowEvent::Resized(physical_size) => {
                    // self.renderer2d.resize(*physical_size);
                },
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    // self.renderer2d.resize(**new_inner_size);
                },
                _ => {}
            },
            Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                now = Instant::now();
                delta = now.duration_since(last).as_millis() as f64 / 1000.0;
                loops.update(&mut self.window, delta, &self.input);
                last = now;

                loops.render(&mut self.window);
            },
            Event::MainEventsCleared => {
                self.window.request_redraw();
            }

            _ => {}
        }});
    }
}

pub trait Loop {
    fn init(&mut self, window: &mut Window);
    fn update(&mut self, window: &mut Window, delta: f64, input: &WinitInputHelper);
    fn render(&mut self, window: &mut Window);
}