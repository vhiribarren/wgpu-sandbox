use crate::draw_context::{DrawContext, Drawable, UniformMatrix4, Vertex};
use cgmath::{Matrix4, SquareMatrix};
use wgpu::util::DeviceExt;
use wgpu::Buffer;

const TRIANGLE: [Vertex; 3] = [
    Vertex {
        position: [0., 1., 0.],
        color: [1., 0., 0.],
    },
    Vertex {
        position: [-1., -1., 0.],
        color: [0., 1., 0.],
    },
    Vertex {
        position: [1., -1., 0.],
        color: [0., 0., 1.],
    },
];

pub struct Triangle {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    transform: Matrix4<f32>,
    transform_buffer: wgpu::Buffer,
    transform_bind_group: wgpu::BindGroup,
}

impl Triangle {
    pub fn init(
        context: &DrawContext,
        vertex_state: wgpu::VertexState,
        fragment_state: wgpu::FragmentState,
    ) -> Self {
        let vertex_buffer = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&[TRIANGLE]),
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
        let transform = Matrix4::identity();
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
        Triangle {
            render_pipeline,
            vertex_buffer,
            transform,
            transform_buffer,
            transform_bind_group,
        }
    }

    pub fn set_transform(&mut self, context: &DrawContext, transform: Matrix4<f32>) {
        self.transform = transform;
        let transform_uniform: UniformMatrix4 = UniformMatrix4(transform.into());
        context.queue.write_buffer(
            &self.transform_buffer,
            0 as wgpu::BufferAddress,
            transform_uniform.as_ref(),
        );
    }
}

impl Drawable for Triangle {
    fn render_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.render_pipeline
    }
    fn vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }
    fn vertex_count(&self) -> usize {
        TRIANGLE.len()
    }
    fn transform_bind_group(&self) -> &wgpu::BindGroup {
        &self.transform_bind_group
    }
}
