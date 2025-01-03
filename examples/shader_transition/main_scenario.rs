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

use demo_cube_wgpu::draw_context::DrawContext;
use demo_cube_wgpu::primitives::cube::CubeOptions;
use demo_cube_wgpu::primitives::{cube, Object3D};
use demo_cube_wgpu::scenario::{Scenario, UpdateInterval};

use web_time::Duration;

const INTERPOLATED_SHADER: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/shaders/default.wgsl"
));

const FLAT_SHADER: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/shaders/flat.wgsl"
));

const ROTATION_DEG_PER_S: f32 = 45.0;
const SHADER_TRANSITION_PERIOD: Duration = Duration::from_secs(1);

pub struct MainScenario {
    pub cube_interpolated: Object3D,
    pub cube_flat: Object3D,
}

impl Scenario for MainScenario {
    fn new(draw_context: &DrawContext) -> Self {
        let interpolated_shader_module = draw_context.create_shader_module(INTERPOLATED_SHADER);
        let flat_shader_module = draw_context.create_shader_module(FLAT_SHADER);
        let cube_interpolated = cube::create_cube(
            draw_context,
            &interpolated_shader_module,
            &interpolated_shader_module,
            Default::default(),
        )
        .unwrap();
        let cube_flat = cube::create_cube(
            draw_context,
            &flat_shader_module,
            &flat_shader_module,
            CubeOptions { with_alpha: true },
        )
        .unwrap();
        Self {
            cube_interpolated,
            cube_flat,
        }
    }
    fn update(&mut self, context: &DrawContext, update_interval: &UpdateInterval) {
        let delta_rotation = ROTATION_DEG_PER_S * update_interval.update_delta.as_secs_f32();
        let transform = cgmath::Matrix4::from_angle_z(cgmath::Deg(delta_rotation))
            * cgmath::Matrix4::from_angle_y(cgmath::Deg(delta_rotation));
        self.cube_interpolated.apply_transform(context, transform);
        self.cube_flat.apply_transform(context, transform);
        self.cube_flat.set_opacity(
            0.5 + f32::sin(
                2. * update_interval.scenario_start.elapsed().as_secs_f32()
                    / SHADER_TRANSITION_PERIOD.as_secs_f32(),
            ) / 2_f32,
        );
    }
    fn render<'drawable, 'render>(
        &'drawable self,
        render_pass: &'render mut wgpu::RenderPass<'drawable>,
    ) {
        self.cube_interpolated.as_ref().render(render_pass);
        self.cube_flat.as_ref().render(render_pass);
    }
}
