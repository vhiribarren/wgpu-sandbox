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

use crate::draw_context::Drawable;
use crate::draw_context::{DrawContext, UniformMatrix4, Vertex};
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
        let transform_uniform: UniformMatrix4 = UniformMatrix4(transform.into());
        #[allow(clippy::unnecessary_cast)]
        context.queue.write_buffer(
            &self.drawable.transform_buffer,
            0 as wgpu::BufferAddress,
            transform_uniform.as_ref(),
        );
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
    let drawable = Drawable::init(context, &TRIANGLE, vertex_state, fragment_state);
    let transform = Matrix4::<f32>::identity();
    Object3D {
        transform,
        drawable,
    }
}
