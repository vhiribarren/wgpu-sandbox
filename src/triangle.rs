use crate::draw_context::{DrawContext, Drawable, Vertex};
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
                    layout: None,
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
        Triangle {
            render_pipeline,
            vertex_buffer,
        }
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
}
