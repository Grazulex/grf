//! Sprite rendering with batching

use glam::{Mat4, Vec2, Vec4};
use wgpu::util::DeviceExt;

use crate::texture::Texture;

/// Vertex data for a sprite
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpriteVertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
    pub color: [f32; 4],
}

impl SpriteVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
        0 => Float32x2,  // position
        1 => Float32x2,  // tex_coords
        2 => Float32x4,  // color
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SpriteVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

/// A rectangular region within a texture (for sprite sheets/atlases)
#[derive(Clone, Copy, Debug)]
pub struct SpriteRegion {
    /// UV coordinates (0.0 to 1.0)
    pub u_min: f32,
    pub v_min: f32,
    pub u_max: f32,
    pub v_max: f32,
}

impl Default for SpriteRegion {
    fn default() -> Self {
        Self {
            u_min: 0.0,
            v_min: 0.0,
            u_max: 1.0,
            v_max: 1.0,
        }
    }
}

impl SpriteRegion {
    /// Create a region from pixel coordinates within a texture
    pub fn from_pixels(x: u32, y: u32, width: u32, height: u32, tex_width: u32, tex_height: u32) -> Self {
        Self {
            u_min: x as f32 / tex_width as f32,
            v_min: y as f32 / tex_height as f32,
            u_max: (x + width) as f32 / tex_width as f32,
            v_max: (y + height) as f32 / tex_height as f32,
        }
    }
}

/// A sprite to be rendered
#[derive(Clone, Debug)]
pub struct Sprite {
    /// Position in world space
    pub position: Vec2,
    /// Size in pixels
    pub size: Vec2,
    /// Origin/pivot point (0.0 to 1.0, default is center)
    pub origin: Vec2,
    /// Rotation in radians
    pub rotation: f32,
    /// Color tint (multiplied with texture)
    pub color: Vec4,
    /// UV region within the texture
    pub region: SpriteRegion,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::new(32.0, 32.0),
            origin: Vec2::new(0.5, 0.5), // Center
            rotation: 0.0,
            color: Vec4::ONE, // White (no tint)
            region: SpriteRegion::default(),
        }
    }
}

impl Sprite {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self {
            position,
            size,
            ..Default::default()
        }
    }

    /// Set the sprite region from pixel coordinates
    pub fn with_region(mut self, x: u32, y: u32, width: u32, height: u32, tex_width: u32, tex_height: u32) -> Self {
        self.region = SpriteRegion::from_pixels(x, y, width, height, tex_width, tex_height);
        self
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    /// Generate the 4 vertices for this sprite
    fn vertices(&self) -> [SpriteVertex; 4] {
        let half_size = self.size * 0.5;
        let origin_offset = (self.origin - Vec2::new(0.5, 0.5)) * self.size;

        // Calculate corners relative to origin
        let corners = [
            Vec2::new(-half_size.x, -half_size.y) - origin_offset, // Bottom-left
            Vec2::new(half_size.x, -half_size.y) - origin_offset,  // Bottom-right
            Vec2::new(half_size.x, half_size.y) - origin_offset,   // Top-right
            Vec2::new(-half_size.x, half_size.y) - origin_offset,  // Top-left
        ];

        // Apply rotation
        let (sin, cos) = self.rotation.sin_cos();
        let rotate = |v: Vec2| -> Vec2 {
            Vec2::new(v.x * cos - v.y * sin, v.x * sin + v.y * cos)
        };

        let positions: [Vec2; 4] = [
            self.position + rotate(corners[0]),
            self.position + rotate(corners[1]),
            self.position + rotate(corners[2]),
            self.position + rotate(corners[3]),
        ];

        let color = self.color.to_array();

        // In screen coordinates (Y down), negative Y is UP on screen
        // So corners[0] (-Y) is visually at TOP, corners[2] (+Y) is visually at BOTTOM
        // UV origin is top-left of image, so we map:
        // - Visual top (corners 0,1 with -Y) -> v_min (top of image)
        // - Visual bottom (corners 2,3 with +Y) -> v_max (bottom of image)
        [
            SpriteVertex {
                position: positions[0].to_array(),
                tex_coords: [self.region.u_min, self.region.v_min], // Top-left visually
                color,
            },
            SpriteVertex {
                position: positions[1].to_array(),
                tex_coords: [self.region.u_max, self.region.v_min], // Top-right visually
                color,
            },
            SpriteVertex {
                position: positions[2].to_array(),
                tex_coords: [self.region.u_max, self.region.v_max], // Bottom-right visually
                color,
            },
            SpriteVertex {
                position: positions[3].to_array(),
                tex_coords: [self.region.u_min, self.region.v_max], // Bottom-left visually
                color,
            },
        ]
    }
}

/// Camera uniform buffer
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

/// Maximum number of sprites per batch
const MAX_SPRITES: usize = 10000;
const MAX_VERTICES: usize = MAX_SPRITES * 4;
const MAX_INDICES: usize = MAX_SPRITES * 6;

/// Batched sprite renderer
pub struct SpriteBatch {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    vertices: Vec<SpriteVertex>,
    sprite_count: usize,
    screen_size: (u32, u32),
}

impl SpriteBatch {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, screen_size: (u32, u32)) -> Self {
        // Create shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Sprite Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/sprite.wgsl").into()),
        });

        // Create camera uniform buffer
        let camera_uniform = CameraUniform {
            view_proj: Self::ortho_matrix(screen_size.0, screen_size.1).to_cols_array_2d(),
        };
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Camera bind group layout
        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // Camera bind group
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        // Texture bind group layout
        let texture_bind_group_layout = Texture::bind_group_layout(device);

        // Pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Sprite Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        // Render pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Sprite Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[SpriteVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, // No culling for 2D sprites
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

        // Create vertex buffer (empty, will be filled each frame)
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Sprite Vertex Buffer"),
            size: (MAX_VERTICES * std::mem::size_of::<SpriteVertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create index buffer (static, always the same pattern)
        let mut indices: Vec<u16> = Vec::with_capacity(MAX_INDICES);
        for i in 0..MAX_SPRITES as u16 {
            let base = i * 4;
            indices.extend_from_slice(&[
                base,
                base + 1,
                base + 2,
                base,
                base + 2,
                base + 3,
            ]);
        }
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sprite Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            camera_buffer,
            camera_bind_group,
            texture_bind_group_layout,
            vertices: Vec::with_capacity(MAX_VERTICES),
            sprite_count: 0,
            screen_size,
        }
    }

    /// Create an orthographic projection matrix for 2D rendering
    fn ortho_matrix(width: u32, height: u32) -> Mat4 {
        // Origin at top-left, Y increases downward (screen coordinates)
        Mat4::orthographic_rh(0.0, width as f32, height as f32, 0.0, -1.0, 1.0)
    }

    /// Update the screen size (call on resize)
    pub fn resize(&mut self, queue: &wgpu::Queue, width: u32, height: u32) {
        self.screen_size = (width, height);
        let camera_uniform = CameraUniform {
            view_proj: Self::ortho_matrix(width, height).to_cols_array_2d(),
        };
        queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[camera_uniform]));
    }

    /// Update the view-projection matrix (call each frame with camera)
    pub fn set_view_matrix(&mut self, queue: &wgpu::Queue, view_matrix: Mat4) {
        let camera_uniform = CameraUniform {
            view_proj: view_matrix.to_cols_array_2d(),
        };
        queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[camera_uniform]));
    }

    /// Begin a new batch
    pub fn begin(&mut self) {
        self.vertices.clear();
        self.sprite_count = 0;
    }

    /// Check if the batch is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sprite_count == 0
    }

    /// Add a sprite to the current batch
    pub fn draw(&mut self, sprite: &Sprite) {
        if self.sprite_count >= MAX_SPRITES {
            log::warn!("SpriteBatch overflow! Maximum {} sprites per batch.", MAX_SPRITES);
            return;
        }

        self.vertices.extend_from_slice(&sprite.vertices());
        self.sprite_count += 1;
    }

    /// End the batch and render all sprites
    pub fn end<'a>(
        &'a mut self,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass<'a>,
        texture_bind_group: &'a wgpu::BindGroup,
    ) {
        if self.sprite_count == 0 {
            return;
        }

        // Upload vertex data
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));

        // Set pipeline and bind groups
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        render_pass.set_bind_group(1, texture_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        // Draw all sprites
        let index_count = (self.sprite_count * 6) as u32;
        render_pass.draw_indexed(0..index_count, 0, 0..1);
    }

    /// Get the texture bind group layout (for creating texture bind groups)
    pub fn texture_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.texture_bind_group_layout
    }
}
