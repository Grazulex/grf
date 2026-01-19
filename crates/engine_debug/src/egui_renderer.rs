//! Egui renderer integration with wgpu
//!
//! Provides a wrapper for rendering egui UI with wgpu.

use egui::Context;
use egui_wgpu::Renderer;
use egui_wgpu::ScreenDescriptor;
use egui_winit::State;
use wgpu::{CommandEncoder, Device, Queue, TextureFormat, TextureView};
use winit::event::WindowEvent;
use winit::window::Window;

/// Egui renderer for wgpu integration
pub struct EguiRenderer {
    /// Egui context
    context: Context,
    /// Egui-winit state for handling input
    state: State,
    /// Egui-wgpu renderer
    renderer: Renderer,
    /// Screen descriptor for rendering
    screen_descriptor: ScreenDescriptor,
}

impl EguiRenderer {
    /// Create a new egui renderer
    pub fn new(
        device: &Device,
        output_format: TextureFormat,
        window: &Window,
        scale_factor: f32,
    ) -> Self {
        let context = Context::default();

        // Configure default style
        let mut style = (*context.style()).clone();
        style.visuals.window_rounding = egui::Rounding::same(4.0);
        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgba_unmultiplied(30, 30, 40, 230);
        context.set_style(style);

        let state = State::new(
            context.clone(),
            egui::ViewportId::ROOT,
            window,
            Some(scale_factor),
            None,
        );

        let renderer = Renderer::new(device, output_format, None, 1);

        let size = window.inner_size();
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [size.width, size.height],
            pixels_per_point: scale_factor,
        };

        Self {
            context,
            state,
            renderer,
            screen_descriptor,
        }
    }

    /// Handle a window event
    /// Returns true if egui consumed the event
    pub fn handle_event(&mut self, window: &Window, event: &WindowEvent) -> bool {
        let response = self.state.on_window_event(window, event);
        response.consumed
    }

    /// Update screen size
    pub fn resize(&mut self, width: u32, height: u32, scale_factor: f32) {
        self.screen_descriptor = ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: scale_factor,
        };
    }

    /// Begin a new frame
    pub fn begin_frame(&mut self, window: &Window) {
        let raw_input = self.state.take_egui_input(window);
        self.context.begin_frame(raw_input);
    }

    /// Get the egui context for drawing UI
    pub fn context(&self) -> &Context {
        &self.context
    }

    /// End the frame and render
    pub fn end_frame_and_render(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        view: &TextureView,
        window: &Window,
    ) {
        let output = self.context.end_frame();

        // Handle platform output (clipboard, cursor, etc.)
        self.state.handle_platform_output(window, output.platform_output);

        // Tessellate shapes
        let paint_jobs = self.context.tessellate(output.shapes, output.pixels_per_point);

        // Update textures
        for (id, image_delta) in &output.textures_delta.set {
            self.renderer.update_texture(device, queue, *id, image_delta);
        }

        // Update buffers
        self.renderer.update_buffers(
            device,
            queue,
            encoder,
            &paint_jobs,
            &self.screen_descriptor,
        );

        // Render
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui_render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load, // Don't clear, draw on top
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.renderer.render(&mut render_pass, &paint_jobs, &self.screen_descriptor);
        }

        // Free textures
        for id in &output.textures_delta.free {
            self.renderer.free_texture(id);
        }
    }
}
