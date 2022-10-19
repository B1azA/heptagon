use std::time::{Instant};

use winit::dpi::{Size, PhysicalSize};
use winit::{
    event_loop::ControlFlow
};

use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::event::Event;
use winit::event::WindowEvent;

// -------- PUBLIC --------
pub mod input;

pub use input::*;

pub type Key = winit::event::VirtualKeyCode;
pub type Window = winit::window::Window;

// ------------------------


/// Struct for controlling app loops and window creation.
pub struct MainLoop {
    event_loop: EventLoop<()>,
    window: Window,
    input: Input,
}

impl MainLoop {
    /// Creates new MainLoop.
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

    /// Returns a reference to the window.
    pub fn window(&self) -> &Window {
        &self.window
    }

    /// Run event loop of an app.
    pub fn run(mut self, mut app: impl App + std::marker::Send + 'static) {
        env_logger::init();

        let mut last = Instant::now();

        self.event_loop.run(move |event, _, control_flow| {
            self.input.update(&event);
            
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => match event {
                    WindowEvent::CloseRequested => { 
                        *control_flow = ControlFlow::Exit;
                    },

                    _ => {}
                },
                Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                    app.render();                   
                },
                Event::MainEventsCleared => {
                    if self.input.mouse_lock {
                        let new_pos = winit::dpi::PhysicalPosition::new(self.window.inner_size().width / 2,
                            self.window.inner_size().height / 2);
                        self.window.set_cursor_position(new_pos).unwrap();
                    }
                    
                    // UPDATE
                    let now = Instant::now();
                    let delta = now.duration_since(last).as_micros() as f32 / 1000000.0;
                    app.update(&mut self.window, delta, &mut self.input);
                    self.input.updated();
                    last = now;
                    
                    // RENDER
                    self.window.request_redraw();
                },

                _ => {}
            }

        });
    }
}

/// Trait for creating struct which can be controlled by an event loop.
pub trait App {
    /// Update function.
    fn update(&mut self, window: &mut Window, delta: f32, input: &mut Input);
    /// Render function.
    fn render(&mut self);
}
