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
use crate::draw_context::{DrawContext, Vertex};
use crate::primitives::Object3D;
use cgmath::Matrix4;
use cgmath::SquareMatrix;

const WHITE_COLOR: [f32; 3] = [1., 1., 1.];

const CUBE_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, 0.5, -0.5],
        color: WHITE_COLOR,
    },
    Vertex {
        position: [0.5, 0.5, -0.5],
        color: WHITE_COLOR,
    },
    Vertex {
        position: [0.5, -0.5, -0.5],
        color: WHITE_COLOR,
    },
    Vertex {
        position: [-0.5, -0.5, -0.5],
        color: WHITE_COLOR,
    },
    Vertex {
        position: [-0.5, 0.5, 0.5],
        color: WHITE_COLOR,
    },
    Vertex {
        position: [0.5, 0.5, 0.5],
        color: WHITE_COLOR,
    },
    Vertex {
        position: [0.5, -0.5, 0.5],
        color: WHITE_COLOR,
    },
    Vertex {
        position: [-0.5, -0.5, 0.5],
        color: WHITE_COLOR,
    },
];

const CUBE_INDICES: &[[u16; 3]] = &[
    // Front
    [0, 2, 1],
    [0, 3, 2],
    // Back
    [4, 5, 6],
    [6, 7, 4],
    // Above
    [4, 0, 1],
    [1, 5, 4],
    // Below
    [7, 2, 3],
    [2, 7, 6],
    // Left side
    [3, 0, 4],
    [4, 7, 3],
    // Right side
    [5, 1, 2],
    [2, 6, 5],
];

pub fn create_cube(
    context: &DrawContext,
    vertex_state: wgpu::VertexState,
    fragment_state: wgpu::FragmentState,
) -> Object3D {
    let drawable = Drawable::init_indexed(
        context,
        CUBE_VERTICES,
        CUBE_INDICES,
        vertex_state,
        fragment_state,
    );
    let transform = Matrix4::<f32>::identity();
    Object3D {
        transform,
        drawable,
    }
}
