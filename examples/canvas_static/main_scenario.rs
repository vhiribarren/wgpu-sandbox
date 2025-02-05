/*
MIT License

Copyright (c) 2025 Vincent Hiribarren

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

use wgpu_lite_wrapper::draw_context::{DrawContext, Drawable, DrawableBuilder};
use wgpu_lite_wrapper::scenario::WinitEventLoopHandler;

const CANVAS_STATIC_SHADER: &str = include_str!("./shader.wgsl");

pub struct MainScenario {
    canvas: Drawable,
}

impl MainScenario {
    pub fn new(draw_context: &DrawContext) -> Self {
        let shader_module = draw_context.create_shader_module(CANVAS_STATIC_SHADER);
        let drawable_builder = DrawableBuilder::new(
            draw_context,
            &shader_module,
            &shader_module,
            wgpu_lite_wrapper::draw_context::DrawModeParams::Direct { vertex_count: 3 },
        );
        let canvas = drawable_builder.build();
        Self { canvas }
    }
}

impl WinitEventLoopHandler for MainScenario {
    fn on_render<'drawable>(&'drawable self, render_pass: &mut wgpu::RenderPass<'drawable>) {
        self.canvas.render(render_pass);
    }
}
