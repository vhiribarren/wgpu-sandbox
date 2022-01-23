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
use std::collections::BTreeSet;
use winit::event::{DeviceEvent, ElementState, KeyboardInput, VirtualKeyCode};

pub struct Camera {
    pub projection: Matrix4<f32>,
    pub view: Matrix4<f32>,
}

impl Camera {
    pub fn orthogonal() -> Self {
        Camera {
            projection: Matrix4::identity(),
            view: Matrix4::identity(),
        }
    }
    pub fn get_camera_matrix(&self) -> Matrix4<f32> {
        self.projection * self.view
    }
    fn move_z(&mut self, val: f32) {
        self.view = Matrix4::from_translation(Vector3::new(0., 0., -val)) * self.view;
    }
    fn move_x(&mut self, val: f32) {
        self.view = Matrix4::from_translation(Vector3::new(-val, 0., 0.)) * self.view;
    }
    fn move_y(&mut self, val: f32) {
        self.view = Matrix4::from_translation(Vector3::new(0., -val, 0.)) * self.view;
    }
}

pub struct WinitCameraAdapter {
    camera: Camera,
    enabled_keys: BTreeSet<VirtualKeyCode>,
    key_speed: f32,
}

impl WinitCameraAdapter {
    const DEFAULT_KEY_SPEED: f32 = 0.03;

    pub fn new(camera: Camera) -> Self {
        WinitCameraAdapter {
            camera,
            enabled_keys: BTreeSet::new(),
            key_speed: Self::DEFAULT_KEY_SPEED,
        }
    }

    pub fn get_camera_matrix(&self) -> Matrix4<f32> {
        self.camera.get_camera_matrix()
    }

    pub fn mouse_event_listener(&mut self, input: &DeviceEvent) {
        dbg!(input);
    }

    pub fn keyboard_event_listener(&mut self, input: &KeyboardInput) {
        match input.virtual_keycode {
            None => {}
            Some(key) => {
                if input.state == ElementState::Pressed {
                    self.enabled_keys.insert(key);
                } else {
                    self.enabled_keys.remove(&key);
                }
            }
        }
    }

    pub fn update(&mut self) {
        for key in self.enabled_keys.iter() {
            match *key {
                VirtualKeyCode::Up => self.camera.move_z(self.key_speed),
                VirtualKeyCode::Down => self.camera.move_z(-self.key_speed),
                VirtualKeyCode::Left => self.camera.move_x(-self.key_speed),
                VirtualKeyCode::Right => self.camera.move_x(self.key_speed),
                VirtualKeyCode::PageUp => self.camera.move_y(self.key_speed),
                VirtualKeyCode::PageDown => self.camera.move_y(-self.key_speed),
                _ => {}
            };
        }
    }
}

impl AsRef<Camera> for WinitCameraAdapter {
    fn as_ref(&self) -> &Camera {
        &self.camera
    }
}
