use crate::scenarios::{Scenario, UpdateInterval};
use crate::triangle::Triangle;
use std::f64::consts::PI;
use std::time::Duration;

const ROTATION_COEFF: f64 = PI / 4.0; // rad/s

pub struct SimpleTriangleRotation {
    //triangle: Triangle,
}

impl SimpleTriangleRotation {
    pub fn new() -> Self {
        todo!()
    }
}

impl Scenario for SimpleTriangleRotation {
    fn update(&mut self, update_interval: &UpdateInterval) {
        //let total_seconds = update_interval.scenario_start.elapsed().as_secs() as f64;
        //let new_rotation = ROTATION_COEFF * total_seconds;
        //let transform = cgmath::Matrix4::from_angle_z(new_rotation);
        //self.triangle.set_transform(transform);
    }

    fn render(&self) {
        todo!()
    }
}
