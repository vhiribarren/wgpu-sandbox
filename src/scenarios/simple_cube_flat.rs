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

use crate::draw_context::DrawContext;
use crate::primitives::{cube, Object3D};
use crate::scenarios::{Scenario, UpdateInterval};

const DEFAULT_SHADER: &str = include_str!("../shaders/flat.wgsl");
const DEFAULT_SHADER_MAIN_FRG: &str = "frg_main";
const DEFAULT_SHADER_MAIN_VTX: &str = "vtx_main";

const ROTATION_DEG_PER_S: f32 = 45.0;

pub struct SimpleCubeFlat {
    pub cube: Object3D,
}

impl SimpleCubeFlat {
    pub fn new(draw_context: &DrawContext) -> Self {
        let default_shader_module =
            draw_context
                .device
                .create_shader_module(&wgpu::ShaderModuleDescriptor {
                    label: Some("Vertex Shader"),
                    source: wgpu::ShaderSource::Wgsl(DEFAULT_SHADER.into()),
                });
        let vertex_state = wgpu::VertexState {
            module: &default_shader_module,
            entry_point: DEFAULT_SHADER_MAIN_VTX,
            buffers: &[draw_context.vertex_buffer_layout.clone()],
        };
        let fragment_state = wgpu::FragmentState {
            module: &default_shader_module,
            entry_point: DEFAULT_SHADER_MAIN_FRG,
            targets: &[wgpu::ColorTargetState {
                format: draw_context.config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            }],
        };
        let cube = cube::create_cube(draw_context, vertex_state, fragment_state);
        Self { cube }
    }
}

impl Scenario for SimpleCubeFlat {
    fn update(&mut self, context: &DrawContext, update_interval: &UpdateInterval) {
        let total_seconds = update_interval.scenario_start.elapsed().as_secs_f32();
        let new_rotation = ROTATION_DEG_PER_S * total_seconds;
        // Translation on z to be in the clipped space (between -w and w) and camera in front of the cube
        let z_translation: cgmath::Matrix4<f32> =
            cgmath::Matrix4::from_translation(cgmath::Vector3::new(0.0, 0.0, 1.0));
        let transform: cgmath::Matrix4<f32> =
            cgmath::Matrix4::from_angle_z(cgmath::Deg(new_rotation));
        self.cube.set_transform(context, transform * z_translation);
    }
    fn render<'drawable, 'render>(
        &'drawable self,
        render_pass: &'render mut wgpu::RenderPass<'drawable>,
    ) {
        self.cube.as_ref().render(render_pass);
    }
}
