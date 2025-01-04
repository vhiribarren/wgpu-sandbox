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

pub mod color;
pub mod cube;
pub mod triangle;

use crate::draw_context::{DrawContext, Drawable};
use cgmath::Matrix4;
use cgmath::SquareMatrix;

pub struct Object3D {
    drawable: Drawable,
    transform: Matrix4<f32>,
    opacity: f32,
}

impl Object3D {
    pub fn from_drawable(drawable: Drawable) -> Self {
        Object3D {
            drawable,
            transform: Matrix4::<f32>::identity(),
            opacity: 1.0,
        }
    }
    pub fn set_transform(&mut self, context: &DrawContext, transform: Matrix4<f32>) {
        self.transform = transform;
        self.drawable.set_transform(context, self.transform);
    }
    pub fn get_transform(&self) -> &Matrix4<f32> {
        &self.transform
    }
    pub fn apply_transform(&mut self, context: &DrawContext, transform: Matrix4<f32>) {
        self.transform = self.transform * transform; // TODO Shouldn't it be the opposite? But in that case that does not work
        self.drawable.set_transform(context, self.transform);
    }
    pub fn set_opacity(&mut self, value: f32) {
        self.opacity = value.clamp(0., 1.);
        self.drawable.set_blend_color_opacity(self.opacity as f64);
    }
    pub fn get_opacity(&self) -> f32 {
        self.opacity
    }
}

impl AsRef<Drawable> for Object3D {
    fn as_ref(&self) -> &Drawable {
        &self.drawable
    }
}
