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
