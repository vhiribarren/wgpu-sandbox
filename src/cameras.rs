/*
MIT License

Copyright (c) 2022 Vincent Hiribarren

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

use cgmath::SquareMatrix;
use cgmath::{Matrix4, Vector3};

pub trait MovableCamera {
    fn move_x(&mut self, val: f32);
    fn move_y(&mut self, val: f32);
    fn move_z(&mut self, val: f32);
    fn look_at(&mut self, val: Vector3<f32>);
}

pub struct OrthogonalCamera {
    transform: Matrix4<f32>,
}

impl OrthogonalCamera {
    pub fn new() -> Self {
        OrthogonalCamera {
            transform: Matrix4::identity(),
        }
    }
}

impl AsRef<[[f32; 4]; 4]> for OrthogonalCamera {
    fn as_ref(&self) -> &[[f32; 4]; 4] {
        self.transform.as_ref()
    }
}

impl MovableCamera for OrthogonalCamera {
    fn move_x(&mut self, val: f32) {
        self.transform = Matrix4::from_translation(Vector3::new(-val, 0., 0.)) * self.transform;
    }
    fn move_y(&mut self, val: f32) {
        self.transform = Matrix4::from_translation(Vector3::new(0., -val, 0.)) * self.transform;
    }

    fn move_z(&mut self, val: f32) {
        self.transform = Matrix4::from_translation(Vector3::new(0., 0., -val)) * self.transform;
    }

    fn look_at(&mut self, val: Vector3<f32>) {
        todo!()
    }
}
