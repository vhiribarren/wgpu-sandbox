mod simple_triangle_rotation;

use std::time::{Duration, Instant};

pub struct UpdateInterval {
    pub scenario_start: Instant,
    pub update_delta: Duration,
}

pub trait Scenario {
    fn update(&mut self, update_interval: &UpdateInterval);
    fn render(&self);
}
