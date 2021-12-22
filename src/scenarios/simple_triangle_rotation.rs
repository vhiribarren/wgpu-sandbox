use crate::draw_context::DrawContext;
use crate::scenarios::{Scenario, UpdateInterval};
use crate::triangle::Triangle;
use crate::Drawable;
use std::borrow::Borrow;
use std::f64::consts::PI;
use std::ops::Deref;
use std::time::Duration;

const ROTATION_DEG_PER_S: f32 = 45.0;

pub struct SimpleTriangleRotation {
    pub triangle: Box<Triangle>,
}

impl SimpleTriangleRotation {
    pub fn new(triangle: Triangle) -> Self {
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

    fn drawables(&self) -> &dyn Drawable {
        self.triangle.as_ref()
    }
}
