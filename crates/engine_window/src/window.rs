//! Window management with winit

use std::sync::Arc;

use winit::{
    dpi::PhysicalSize,
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window as WinitWindow, WindowBuilder},
};

use crate::{DEFAULT_HEIGHT, DEFAULT_TITLE, DEFAULT_WIDTH};

/// Configuration for creating a window
#[derive(Debug, Clone)]
pub struct WindowConfig {
    /// Window title
    pub title: String,
    /// Window width in pixels
    pub width: u32,
    /// Window height in pixels
    pub height: u32,
    /// Whether the window is resizable
    pub resizable: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: DEFAULT_TITLE.to_string(),
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            resizable: true,
        }
    }
}

impl WindowConfig {
    /// Create a new window config with the given title
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    /// Set the window size
    #[must_use]
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set whether the window is resizable
    #[must_use]
    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }
}

/// Window wrapper that manages the winit window and event loop
pub struct Window {
    config: WindowConfig,
}

impl Window {
    /// Create a new window with the given configuration
    #[must_use]
    pub fn new(config: WindowConfig) -> Self {
        Self { config }
    }

    /// Run the window event loop with the given application handler
    ///
    /// This function blocks until the window is closed.
    ///
    /// # Errors
    /// Returns an error if the event loop fails to start
    pub fn run<A: App + 'static>(self, mut app: A) -> Result<(), winit::error::EventLoopError> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);

        let window = Arc::new(
            WindowBuilder::new()
                .with_title(&self.config.title)
                .with_inner_size(PhysicalSize::new(self.config.width, self.config.height))
                .with_resizable(self.config.resizable)
                .build(&event_loop)?,
        );

        log::info!(
            "Window created: {}x{}",
            self.config.width,
            self.config.height
        );

        app.init(Arc::clone(&window));

        let window_clone = Arc::clone(&window);
        event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent { event, .. } => {
                    // Give app a chance to handle the event first (e.g., for egui)
                    let consumed = app.on_window_event(&event);

                    // If not consumed, handle built-in events
                    if !consumed {
                        match event {
                            WindowEvent::CloseRequested => {
                                if app.on_close_requested() {
                                    log::info!("Window close requested, exiting...");
                                    elwt.exit();
                                }
                            }

                            WindowEvent::Resized(size) => {
                                log::debug!("Window resized to {}x{}", size.width, size.height);
                                app.on_resize(size.width, size.height);
                            }

                            WindowEvent::KeyboardInput { event, .. } => {
                                app.on_keyboard_event(&event);
                            }

                            WindowEvent::RedrawRequested => {
                                app.update();
                                app.render();
                                app.end_frame();
                            }

                            _ => {}
                        }
                    } else {
                        // Even if consumed, handle RedrawRequested
                        if let WindowEvent::RedrawRequested = event {
                            app.update();
                            app.render();
                            app.end_frame();
                        }
                    }
                }

                Event::AboutToWait => {
                    // Request redraw to keep the loop going
                    window_clone.request_redraw();
                }

                _ => {}
            }
        })
    }
}

/// Trait for applications that run in a window
pub trait App {
    /// Called once when the window is created
    /// The Arc<Window> can be cloned and stored for later use (e.g., for wgpu)
    fn init(&mut self, window: Arc<WinitWindow>);

    /// Called every frame to update game logic
    fn update(&mut self);

    /// Called every frame to render
    fn render(&mut self);

    /// Called at the end of each frame (for input state transitions)
    fn end_frame(&mut self) {}

    /// Called for window events (for egui integration etc.)
    /// Return true if the event was consumed and should not be processed further
    fn on_window_event(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    /// Called when a keyboard event occurs
    fn on_keyboard_event(&mut self, event: &KeyEvent) {
        let _ = event;
    }

    /// Called when the window is resized
    fn on_resize(&mut self, width: u32, height: u32) {
        let _ = (width, height);
    }

    /// Called when the window requests to close
    /// Return true to allow closing, false to prevent it
    fn on_close_requested(&mut self) -> bool {
        true
    }
}
