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

use cgmath::{vec3, Matrix4, Vector3};
use cgmath::{Ortho, Point3};
use log::debug;
use std::collections::BTreeSet;
use winit::event::{DeviceEvent, ElementState, KeyboardInput, VirtualKeyCode};

pub fn camera_orthogonal_default() -> Camera {
    Camera::orthogonal(
        16.0,
        9.0,
        Point3 {
            x: 0.0,
            y: 0.0,
            z: -10.0,
        },
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
    )
}

#[derive(Debug)]
pub struct Camera {
    pub projection: Matrix4<f32>,
    pub view: Matrix4<f32>,
}

impl Camera {
    pub fn orthogonal(
        width: f32,
        height: f32,
        eye: Point3<f32>,
        center: Point3<f32>,
        up: Vector3<f32>,
    ) -> Self {
        Camera {
            projection: Matrix4::from(Ortho {
                left: -width / 2.0,
                right: width / 2.0,
                bottom: -height / 2.0,
                top: height / 2.0,
                near: 0.,
                far: 1000.0,
            }),
            view: Matrix4::look_at_lh(eye, center, up),
        }
    }
    pub fn get_camera_matrix(&self) -> Matrix4<f32> {
        let to_rh_coords = Matrix4::from_nonuniform_scale(1., 1., -1.);
        let to_webgpu_ndc_coords = Matrix4::from_translation(vec3(0., 0., 0.5))
            * Matrix4::from_nonuniform_scale(1., 1., 0.5);
        to_webgpu_ndc_coords * self.projection * to_rh_coords * self.view
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

    pub fn mouse_event_listener(&mut self, _input: &DeviceEvent) {}

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
        if self.enabled_keys.is_empty() {
            return;
        }
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
        debug!("{:?}", -self.as_ref().view);
    }
}

impl AsRef<Camera> for WinitCameraAdapter {
    fn as_ref(&self) -> &Camera {
        &self.camera
    }
}
