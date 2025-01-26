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

use crate::draw_context::{DrawContext, DrawableBuilder, Uniform};
use crate::primitives::Object3D;

use super::{color, M4X4_ID_UNIFORM};

#[rustfmt::skip]
const TRIANGLE_GEOMETRY: &[[f32; 3]] = &[
    [0., 2., 0.],
    [-1.732, -1.0, 0.],
    [1.732, -1.0, 0.],
];
#[rustfmt::skip]
const TRIANGLE_COLOR: &[[f32;3]] = &[
    color::COLOR_RED,
    color::COLOR_GREEN,
    color::COLOR_BLUE,
];

const TRIANGLE_VERTEX_COUNT: u32 = TRIANGLE_GEOMETRY.len() as u32;

pub fn create_equilateral_triangle(
    context: &DrawContext,
    vtx_module: &wgpu::ShaderModule,
    frg_module: &wgpu::ShaderModule,
) -> Result<Object3D, anyhow::Error> {
    let camera_uniform = Uniform::new(context, M4X4_ID_UNIFORM);
    let transform_uniform = Uniform::new(context, M4X4_ID_UNIFORM);

    let mut drawable_builder = DrawableBuilder::new(
        context,
        vtx_module,
        frg_module,
        crate::draw_context::DrawModeParams::Direct {
            vertex_count: TRIANGLE_VERTEX_COUNT,
        },
    );
    drawable_builder
        .add_attribute(
            0,
            wgpu::VertexStepMode::Vertex,
            TRIANGLE_GEOMETRY,
            wgpu::VertexFormat::Float32x3,
        )?
        .add_attribute(
            1,
            wgpu::VertexStepMode::Vertex,
            TRIANGLE_COLOR,
            wgpu::VertexFormat::Float32x3,
        )?
        .add_uniform(0, 0, &camera_uniform)?
        .add_uniform(1, 0, &transform_uniform)?;
    let drawable = drawable_builder.build();
    Ok(Object3D::from_drawable(drawable, transform_uniform))
}
