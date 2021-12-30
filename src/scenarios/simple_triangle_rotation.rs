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
use crate::draw_context::Drawable;
use crate::primitive::Object3D;
use crate::scenarios::{Scenario, UpdateInterval};
use std::iter::once;

const ROTATION_DEG_PER_S: f32 = 45.0;

pub struct SimpleTriangleRotation {
    pub triangle: Box<Object3D>,
}

impl SimpleTriangleRotation {
    pub fn new(triangle: Object3D) -> Self {
        SimpleTriangleRotation {
            triangle: Box::new(triangle),
        }
    }
}

impl Scenario for SimpleTriangleRotation {
    fn update(&mut self, context: &DrawContext, update_interval: &UpdateInterval) {
        let total_seconds = update_interval.scenario_start.elapsed().as_secs_f32();
        let new_rotation = ROTATION_DEG_PER_S * total_seconds;
        let transform: cgmath::Matrix4<f32> =
            cgmath::Matrix4::from_angle_z(cgmath::Deg(new_rotation));
        self.triangle.set_transform(context, transform);
    }
    fn drawables<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Drawable> + 'a> {
        Box::new(once((*self.triangle).as_ref()))
    }
}
