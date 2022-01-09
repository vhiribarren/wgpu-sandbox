use crate::draw_context::Drawable;
use crate::draw_context::{DrawContext, Vertex};
use cgmath::Matrix4;
use cgmath::SquareMatrix;

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

pub struct Object3D {
    drawable: Drawable,
    transform: Matrix4<f32>,
}

impl Object3D {
    pub fn set_transform(&mut self, context: &DrawContext, transform: Matrix4<f32>) {
        self.transform = transform;
        self.drawable.set_transform(context, self.transform);
    }
    #[allow(dead_code)]
    pub fn get_transform(&self) -> &Matrix4<f32> {
        &self.transform
    }
}

impl AsRef<Drawable> for Object3D {
    fn as_ref(&self) -> &Drawable {
        &self.drawable
    }
}

pub fn create_triangle(
    context: &DrawContext,
    vertex_state: wgpu::VertexState,
    fragment_state: wgpu::FragmentState,
) -> Object3D {
    let drawable = Drawable::init_direct(context, &TRIANGLE, vertex_state, fragment_state);
    let transform = Matrix4::<f32>::identity();
    Object3D {
        transform,
        drawable,
    }
}
