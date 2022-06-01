use std::time::{Instant};

use winit::dpi::{Size, PhysicalSize};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{ WindowBuilder},
};

pub mod input;

pub use input::*;
pub use winit::window::Window;
pub use winit;
pub use winit::event::VirtualKeyCode;
pub use crate::rendering::renderer::*;
pub use winit_input_helper::WinitInputHelper;
pub use glam;

pub struct MainLoop {
    event_loop: EventLoop<()>,
    pub window: Window,
    input: Input,
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
            input: Input::new(),
        }
    }

    pub fn get_render_belongings(&self) -> (wgpu::Surface, wgpu::Device, wgpu::Queue, wgpu::SurfaceConfiguration) {
        async_std::task::block_on(self.get_render_belongings_async())
    }

    async fn get_render_belongings_async(&self) -> (wgpu::Surface, wgpu::Device, wgpu::Queue, wgpu::SurfaceConfiguration) {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&self.window) };
        let size = self.window.inner_size();
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
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        (surface, device, queue, config)
    }

    pub fn run(mut self, mut loops: impl Loop + std::marker::Send + 'static) {
        env_logger::init();
        loops.init(&mut self.window);

        let mut last = Instant::now();

        self.event_loop.run(move |event, _, control_flow| {
            self.input.update(&event);
            
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => match event {
                    WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit },
                    _ => {}
                },
                Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                    loops.render(&mut self.window);                    
                },
                Event::MainEventsCleared => {
                    if self.input.mouse_lock {
                        let new_pos = winit::dpi::PhysicalPosition::new(self.window.inner_size().width / 2, 
                        self.window.inner_size().height / 2);
                        let mouse_pos = self.input.input_helper.mouse();
                        self.window.set_cursor_position(new_pos).unwrap();
                        
                        if let Some(pos) = mouse_pos {
                            self.input.mouse_difference.0 = pos.0 - new_pos.x as f32;
                            self.input.mouse_difference.1 = pos.1  - new_pos.y as f32;
                        } else {
                            self.input.mouse_difference = (0.0, 0.0);
                        }
                    }

                    // UPDATE
                    let now = Instant::now();
                    let delta = now.duration_since(last).as_millis() as f64 / 1000.0;
                    loops.update(&mut self.window, delta, &mut self.input);
                    last = now;

                    // RENDER
                    self.window.request_redraw();
                },

                _ => {}
            }
        });
    }
}

pub trait Loop {
    fn init(&mut self, window: &mut Window);
    fn update(&mut self, window: &mut Window, delta: f64, input: &mut Input);
    fn render(&mut self, window: &mut Window);
}