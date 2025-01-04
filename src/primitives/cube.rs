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

use crate::draw_context::DrawContext;
use crate::draw_context::DrawableBuilder;
use crate::draw_context::IndexData;
use crate::primitives::color;
use crate::primitives::Object3D;

#[rustfmt::skip]
const CUBE_GEOMETRY: &[[f32; 3]] = &[
    [-0.5, 0.5, -0.5],
    [0.5, 0.5, -0.5],
    [0.5, -0.5, -0.5],
    [-0.5, -0.5, -0.5],
    [-0.5, 0.5, 0.5],
    [0.5, 0.5, 0.5],
    [0.5, -0.5, 0.5],
    [-0.5, -0.5, 0.5],
];
#[rustfmt::skip]
const CUBE_INDICES: &[u16] = &[
    // Front
    0, 2, 1,
    0, 3, 2,
    // Back
    5, 7, 4,
    5, 6, 7,
    // Above
    4, 1, 5,
    4, 0, 1,
    // Below
    6, 3, 7,
    6, 2, 3,
    // Left side
    7, 0, 4,
    7, 3, 0,
    // Right side
    2, 5, 1,
    2, 6, 5,
];
#[rustfmt::skip]
const CUBE_COLOR: &[[f32; 3]] = &[
    color::COLOR_WHITE, 
    color::COLOR_BLACK, 
    color::COLOR_RED, 
    color::COLOR_GREEN, 
    color::COLOR_BLUE, 
    color::COLOR_YELLOW, 
    color::COLOR_CYAN, 
    color::COLOR_MAGENTA, 
];

pub struct CubeOptions {
    pub with_alpha: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for CubeOptions {
    fn default() -> Self {
        Self { with_alpha: false }
    }
}

pub fn create_cube(
    context: &DrawContext,
    vtx_module: &wgpu::ShaderModule,
    frg_module: &wgpu::ShaderModule,
    options: CubeOptions,
) -> Result<Object3D, anyhow::Error> {
    let mut drawable_builder = DrawableBuilder::new(context, vtx_module, frg_module);
    drawable_builder
        .add_attribute(
            0,
            wgpu::VertexStepMode::Vertex,
            CUBE_GEOMETRY,
            wgpu::VertexFormat::Float32x3,
        )?
        .add_attribute(
            1,
            wgpu::VertexStepMode::Vertex,
            CUBE_COLOR,
            wgpu::VertexFormat::Float32x3,
        )?;
    if options.with_alpha {
        drawable_builder.set_blend_option(wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::Constant,
                dst_factor: wgpu::BlendFactor::OneMinusConstant,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: Default::default(),
        });
    }
    let drawable = drawable_builder.build_for_indexed_draw(IndexData::U16(CUBE_INDICES));
    //with_index_count? soit vertex count, soit indices .set_index_count(CUBE_VERTEX_COUNT);
    Ok(Object3D::from_drawable(drawable))
}
