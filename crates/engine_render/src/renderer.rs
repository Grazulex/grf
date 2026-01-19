//! GPU Renderer using wgpu

use std::sync::Arc;
use winit::window::Window;

use crate::sprite::{Sprite, SpriteBatch};
use crate::stats::RenderStats;
use crate::texture::Texture;
use crate::CLEAR_COLOR;

/// Holds all wgpu state for rendering
pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: (u32, u32),
    // Sprite rendering
    sprite_batch: SpriteBatch,
    // Default white texture for solid colors
    #[allow(dead_code)]
    white_texture: Texture,
    white_bind_group: wgpu::BindGroup,
    // Render statistics for profiling
    stats: RenderStats,
}

impl Renderer {
    /// Create a new renderer from a winit window
    ///
    /// # Panics
    /// Panics if wgpu initialization fails
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let size = (size.width.max(1), size.height.max(1));

        // Create wgpu instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Create surface from window
        let surface = instance
            .create_surface(window)
            .expect("Failed to create surface");

        // Request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find a suitable GPU adapter");

        log::info!("GPU: {}", adapter.get_info().name);
        log::info!("Backend: {:?}", adapter.get_info().backend);

        // Request device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("GRF Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        // Configure surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0,
            height: size.1,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        // Create sprite batch
        let sprite_batch = SpriteBatch::new(&device, surface_format, size);

        // Create default white texture
        let white_texture = Texture::white_pixel(&device, &queue);
        let white_bind_group = white_texture.bind_group(&device, sprite_batch.texture_bind_group_layout());

        log::info!("Renderer initialized: {}x{}", size.0, size.1);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            sprite_batch,
            white_texture,
            white_bind_group,
            stats: RenderStats::new(),
        }
    }

    /// Resize the renderer surface
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.size = (width, height);
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.sprite_batch.resize(&self.queue, width, height);
            log::debug!("Renderer resized: {}x{}", width, height);
        }
    }

    /// Get the current surface size
    #[must_use]
    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    /// Get a reference to the device
    #[must_use]
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    /// Get a reference to the queue
    #[must_use]
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    /// Get the surface format
    #[must_use]
    pub fn surface_format(&self) -> wgpu::TextureFormat {
        self.config.format
    }

    /// Get the texture bind group layout for creating texture bind groups
    #[must_use]
    pub fn texture_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        self.sprite_batch.texture_bind_group_layout()
    }

    /// Get the current render statistics
    #[must_use]
    pub fn stats(&self) -> &RenderStats {
        &self.stats
    }

    /// Create a texture from a file path
    pub fn load_texture(&self, path: impl AsRef<std::path::Path>) -> Result<Texture, image::ImageError> {
        Texture::from_path(&self.device, &self.queue, path, None)
    }

    /// Create a bind group for a texture
    pub fn create_texture_bind_group(&self, texture: &Texture) -> wgpu::BindGroup {
        texture.bind_group(&self.device, self.texture_bind_group_layout())
    }

    /// Set the camera view matrix for rendering
    /// Call this each frame before drawing sprites
    pub fn set_camera(&mut self, camera: &crate::Camera2D) {
        self.sprite_batch.set_view_matrix(&self.queue, camera.view_matrix());
    }

    /// Begin a new frame for rendering
    ///
    /// Returns Ok(Frame) on success, Err if the surface is lost
    pub fn begin_frame(&mut self) -> Result<Frame, wgpu::SurfaceError> {
        // Reset stats for new frame
        self.stats.reset();

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.sprite_batch.begin();

        Ok(Frame {
            output,
            view,
            encoder,
        })
    }

    /// Draw a sprite (batched)
    pub fn draw_sprite(&mut self, sprite: &Sprite) {
        self.sprite_batch.draw(sprite);
    }

    /// Flush sprites to the frame (call before overlay rendering)
    /// This clears the screen and renders all batched sprites
    pub fn flush_sprites(&mut self, frame: &mut Frame, texture_bind_group: Option<&wgpu::BindGroup>) {
        // Use white texture if none provided
        let bind_group = texture_bind_group.unwrap_or(&self.white_bind_group);

        // Record stats before flushing
        let sprite_count = self.sprite_batch.sprite_count();
        if sprite_count > 0 {
            self.stats.record_draw(sprite_count);
            self.stats.record_texture_bind();
        }

        {
            let mut render_pass = frame.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Sprite Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(CLEAR_COLOR),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.sprite_batch.end(&self.queue, &mut render_pass, bind_group);
        }

        // Reset batch so end_frame knows sprites were already rendered
        self.sprite_batch.begin();
    }

    /// End the frame and present (submits commands and presents)
    pub fn end_frame(&mut self, frame: Frame, texture_bind_group: Option<&wgpu::BindGroup>) {
        // Use white texture if none provided
        let bind_group = texture_bind_group.unwrap_or(&self.white_bind_group);

        // Only render sprites here if flush_sprites wasn't called
        // Check if batch is non-empty
        if !self.sprite_batch.is_empty() {
            // Record stats for this batch
            let sprite_count = self.sprite_batch.sprite_count();
            self.stats.record_draw(sprite_count);
            self.stats.record_texture_bind();

            let mut frame = frame;
            {
                let mut render_pass = frame.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(CLEAR_COLOR),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                self.sprite_batch.end(&self.queue, &mut render_pass, bind_group);
            }

            self.queue.submit(std::iter::once(frame.encoder.finish()));
            frame.output.present();
        } else {
            // Sprites already flushed, just submit
            self.queue.submit(std::iter::once(frame.encoder.finish()));
            frame.output.present();
        }
    }

    /// Simple render method for backwards compatibility (renders a colored quad)
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.begin_frame()?;

        // Draw a test sprite (colored rectangle) in the center
        let sprite = Sprite::new(
            glam::Vec2::new(self.size.0 as f32 / 2.0, self.size.1 as f32 / 2.0),
            glam::Vec2::new(100.0, 100.0),
        )
        .with_color(glam::Vec4::new(1.0, 0.5, 0.2, 1.0)); // Orange

        self.draw_sprite(&sprite);
        self.end_frame(frame, None);

        Ok(())
    }
}

/// A frame in progress
pub struct Frame {
    output: wgpu::SurfaceTexture,
    /// The texture view for rendering
    pub view: wgpu::TextureView,
    /// The command encoder for recording GPU commands
    pub encoder: wgpu::CommandEncoder,
}
