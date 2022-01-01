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

use crate::Scenario;
use anyhow::anyhow;
use log::debug;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UniformMatrix4(pub [[f32; 4]; 4]);

impl UniformMatrix4 {
    pub fn identity() -> Self {
        use cgmath::SquareMatrix;
        Self(cgmath::Matrix4::identity().into())
    }
}

impl AsRef<[u8]> for UniformMatrix4 {
    fn as_ref(&self) -> &[u8] {
        bytemuck::cast_slice(&self.0)
    }
}

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

pub struct Drawable {
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub vertex_count: u32,
    pub transform_buffer: wgpu::Buffer,
    pub transform_bind_group: wgpu::BindGroup,
}

impl Drawable {
    pub fn init(
        context: &DrawContext,
        vertex_slice: &[Vertex],
        vertex_state: wgpu::VertexState,
        fragment_state: wgpu::FragmentState,
    ) -> Self {
        let vertex_count = vertex_slice.len() as u32;
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
                        clamp_depth: false,
                        polygon_mode: wgpu::PolygonMode::Fill, // wgpu::PolygonMode::Line
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: Default::default(),
                });
        let transform_buffer =
            context
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Camera Buffer"),
                    contents: crate::draw_context::UniformMatrix4::identity().as_ref(),
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
        Drawable {
            render_pipeline,
            vertex_count,
            vertex_buffer,
            transform_buffer,
            transform_bind_group,
        }
    }

    pub fn render<'drawable, 'render>(
        &'drawable self,
        render_pass: &'render mut wgpu::RenderPass<'drawable>,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.transform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.vertex_count, 0..1);
    }
}

pub struct DrawContext<'a> {
    _adapter: wgpu::Adapter,
    surface: wgpu::Surface,
    pub queue: wgpu::Queue,
    pub transform_bind_group_layout: wgpu::BindGroupLayout,
    pub device: wgpu::Device,
    pub vertex_buffer_layout: wgpu::VertexBufferLayout<'a>,
    pub config: wgpu::SurfaceConfiguration,
    pub pipeline_layout: wgpu::PipelineLayout,
}

impl DrawContext<'_> {
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
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&transform_bind_group_layout],
            push_constant_ranges: &[],
        });
        Ok(DrawContext {
            _adapter: adapter,
            surface,
            device,
            queue,
            config,
            transform_bind_group_layout,
            vertex_buffer_layout,
            pipeline_layout,
        })
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
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        scene.render(&mut render_pass);

        drop(render_pass);
        let command_buffers = std::iter::once(encoder.finish());
        self.queue.submit(command_buffers);
        displayed_texture.present();
        Ok(())
    }
}
