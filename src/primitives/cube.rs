/*
MIT License

Copyright (c) 2021, 2022, 2024, 2025 Vincent Hiribarren

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

const COLOR_WHITE: [f32; 3] = [1., 1., 1.];
const COLOR_BLACK: [f32; 3] = [0., 0., 0.];
const COLOR_RED: [f32; 3] = [1., 0., 0.];
const COLOR_GREEN: [f32; 3] = [0., 1., 0.];
const COLOR_BLUE: [f32; 3] = [0., 0., 1.];
const COLOR_YELLOW: [f32; 3] = [1., 1., 0.];
const COLOR_CYAN: [f32; 3] = [0., 1., 1.];
const COLOR_MAGENTA: [f32; 3] = [1., 0., 1.];

const CUBE_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, 0.5, -0.5],
        color: COLOR_MAGENTA,
    },
    Vertex {
        position: [0.5, 0.5, -0.5],
        color: COLOR_WHITE,
    },
    Vertex {
        position: [0.5, -0.5, -0.5],
        color: COLOR_RED,
    },
    Vertex {
        position: [-0.5, -0.5, -0.5],
        color: COLOR_BLACK,
    },
    Vertex {
        position: [-0.5, 0.5, 0.5],
        color: COLOR_BLUE,
    },
    Vertex {
        position: [0.5, 0.5, 0.5],
        color: COLOR_YELLOW,
    },
    Vertex {
        position: [0.5, -0.5, 0.5],
        color: COLOR_CYAN,
    },
    Vertex {
        position: [-0.5, -0.5, 0.5],
        color: COLOR_GREEN,
    },
];

const CUBE_INDICES: &[[u16; 3]] = &[
    // Front
    [0, 2, 1],
    [0, 3, 2],
    // Back
    [5, 7, 4],
    [5, 6, 7],
    // Above
    [4, 1, 5],
    [4, 0, 1],
    // Below
    [6, 3, 7],
    [6, 2, 3],
    // Left side
    [7, 0, 4],
    [7, 3, 0],
    // Right side
    [2, 5, 1],
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
    Object3D::from_drawable(drawable)
}
