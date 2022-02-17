/*
MIT License

Copyright (c) 2021, 2022 Vincent Hiribarren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use crate::draw_context::Drawable::{Direct, Indexed};
use crate::scenario::Scenario;
use anyhow::anyhow;
use log::debug;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{BindGroupLayoutDescriptor, BindingType, BufferBindingType, ShaderStages};

const M4X4_ID_UNIFORM: [[f32; 4]; 4] = [
    [1., 0., 0., 0.],
    [0., 1., 0., 0.],
    [0., 0., 1., 0.],
    [0., 0., 0., 1.],
];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
            ],
        }
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: [0., 0., 0.],
            color: [1., 1., 1.],
        }
    }
}

struct BaseDrawable {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    transform_buffer: wgpu::Buffer,
    transform_bind_group: wgpu::BindGroup,
    blend_color_opacity: wgpu::Color,
}

pub struct DirectRenderingDrawable {
    base: BaseDrawable,
    vertex_count: u32,
}

pub struct IndexedRenderingDrawable {
    base: BaseDrawable,
    index_buffer: wgpu::Buffer,
    index_count: u32,
}

pub enum Drawable {
    Direct(DirectRenderingDrawable),
    Indexed(IndexedRenderingDrawable),
}

impl Drawable {
    pub fn init_direct(
        context: &DrawContext,
        vertex_slice: &[Vertex],
        vertex_state: wgpu::VertexState,
        fragment_state: wgpu::FragmentState,
    ) -> Self {
        let vertex_count = vertex_slice.len() as u32;
        let base = Self::init_base(context, vertex_slice, vertex_state, fragment_state);
        Direct(DirectRenderingDrawable { base, vertex_count })
    }

    pub fn init_indexed(
        context: &DrawContext,
        vertex_slice: &[Vertex],
        vertex_indices: &[[u16; 3]],
        vertex_state: wgpu::VertexState,
        fragment_state: wgpu::FragmentState,
    ) -> Self {
        let base = Self::init_base(context, vertex_slice, vertex_state, fragment_state);
        let index_buffer = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(vertex_indices),
                usage: wgpu::BufferUsages::INDEX,
            });
        let index_count = 3 * vertex_indices.len() as u32;
        Indexed(IndexedRenderingDrawable {
            base,
            index_buffer,
            index_count,
        })
    }

    fn init_base(
        context: &DrawContext,
        vertex_slice: &[Vertex],
        vertex_state: wgpu::VertexState,
        fragment_state: wgpu::FragmentState,
    ) -> BaseDrawable {
        let vertex_buffer = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertex_slice),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let render_pipeline =
            context
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&context.pipeline_layout),
                    vertex: vertex_state,
                    fragment: Some(fragment_state),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        unclipped_depth: false,
                        polygon_mode: wgpu::PolygonMode::Fill, // wgpu::PolygonMode::Line
                        conservative: false,
                    },
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: wgpu::TextureFormat::Depth32Float,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::LessEqual,
                        stencil: Default::default(),
                        bias: Default::default(),
                    }),
                    multisample: Default::default(),
                    multiview: None,
                });
        let transform_buffer =
            context
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Camera Buffer"),
                    contents: bytemuck::cast_slice(&M4X4_ID_UNIFORM),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                });
        let transform_bind_group = context
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Transform bind group"),
                layout: &context.transform_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: transform_buffer.as_entire_binding(),
                }],
            });
        let blend_color_opacity = wgpu::Color::WHITE;
        BaseDrawable {
            render_pipeline,
            vertex_buffer,
            transform_buffer,
            transform_bind_group,
            blend_color_opacity,
        }
    }

    pub fn set_transform(&mut self, context: &DrawContext, transform: impl AsRef<[[f32; 4]; 4]>) {
        #[allow(clippy::unnecessary_cast)]
        context.queue.write_buffer(
            &self.as_ref().transform_buffer,
            0 as wgpu::BufferAddress,
            bytemuck::cast_slice(transform.as_ref()),
        );
    }

    pub fn set_blend_color_opacity(&mut self, value: f64) {
        let value = value.clamp(0., 1.);
        self.as_mut().blend_color_opacity = wgpu::Color {
            r: value,
            g: value,
            b: value,
            a: 1.0,
        }
    }

    pub fn render<'drawable, 'render>(
        &'drawable self,
        render_pass: &'render mut wgpu::RenderPass<'drawable>,
    ) {
        let base = self.as_ref();
        render_pass.set_pipeline(&base.render_pipeline);
        render_pass.set_bind_group(1, &base.transform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, base.vertex_buffer.slice(..));
        render_pass.set_blend_constant(base.blend_color_opacity);
        dbg!(base.blend_color_opacity);
        match self {
            Drawable::Direct(d) => {
                render_pass.draw(0..d.vertex_count, 0..1);
            }
            Drawable::Indexed(d) => {
                render_pass.set_index_buffer(d.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..d.index_count, 0, 0..1);
            }
        };
    }
}

impl AsRef<BaseDrawable> for Drawable {
    fn as_ref(&self) -> &BaseDrawable {
        match self {
            Self::Direct(d) => &d.base,
            Self::Indexed(d) => &d.base,
        }
    }
}

impl AsMut<BaseDrawable> for Drawable {
    fn as_mut(&mut self) -> &mut BaseDrawable {
        match self {
            Self::Direct(d) => &mut d.base,
            Self::Indexed(d) => &mut d.base,
        }
    }
}

pub struct DrawContext<'a> {
    _adapter: wgpu::Adapter,
    surface: wgpu::Surface,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    #[allow(dead_code)]
    depth_texture: wgpu::Texture,
    depth_texture_view: wgpu::TextureView,
    pub queue: wgpu::Queue,
    pub transform_bind_group_layout: wgpu::BindGroupLayout,
    pub device: wgpu::Device,
    pub vertex_buffer_layout: wgpu::VertexBufferLayout<'a>,
    pub config: wgpu::SurfaceConfiguration,
    pub pipeline_layout: wgpu::PipelineLayout,
}

impl DrawContext<'_> {
    pub const BIND_GROUP_INDEX_CAMERA: u32 = 0;

    pub async fn new<'a, 'b>(
        window_handler: &'a impl raw_window_handle::HasRawWindowHandle,
        width: u32,
        height: u32,
    ) -> anyhow::Result<DrawContext<'b>> {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window_handler) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: Default::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or_else(|| anyhow!("Could not create WebGPU adapter"))?;
        debug!("{:?}", adapter);
        debug!("{:?}", adapter.features());
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device Descriptor"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_webgl2_defaults(),
                },
                None,
            )
            .await?;
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);
        let vertex_buffer_layout = Vertex::vertex_buffer_layout();
        let transform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Transform bind group"),
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
        let camera_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&M4X4_ID_UNIFORM),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        });
        let camera_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout, &transform_bind_group_layout],
            push_constant_ranges: &[],
        });
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // | wgpu::TextureUsages::TEXTURE_BINDING,
        });
        let depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        Ok(DrawContext {
            _adapter: adapter,
            surface,
            device,
            queue,
            config,
            camera_buffer,
            camera_bind_group,
            transform_bind_group_layout,
            vertex_buffer_layout,
            pipeline_layout,
            depth_texture,
            depth_texture_view,
        })
    }

    pub fn set_projection(&self, transform: impl AsRef<[[f32; 4]; 4]>) {
        #[allow(clippy::unnecessary_cast)]
        self.queue.write_buffer(
            &self.camera_buffer,
            0 as wgpu::BufferAddress,
            bytemuck::cast_slice(transform.as_ref()),
        );
    }

    pub fn render_scene<T: Scenario>(&self, scene: &T) -> anyhow::Result<()> {
        let displayed_texture = self.surface.get_current_texture()?;
        let view = displayed_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Encoder"),
            });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.5,
                        b: 0.5,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });
        render_pass.set_bind_group(Self::BIND_GROUP_INDEX_CAMERA, &self.camera_bind_group, &[]);
        scene.render(&mut render_pass);

        drop(render_pass);
        let command_buffers = std::iter::once(encoder.finish());
        self.queue.submit(command_buffers);
        displayed_texture.present();
        Ok(())
    }
}
