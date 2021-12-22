pub mod simple_triangle_rotation;

use crate::draw_context::DrawContext;
use crate::Drawable;
use std::time::{Duration, Instant};

pub struct UpdateInterval {
    pub scenario_start: Instant,
    pub update_delta: Duration,
}

pub trait Scenario {
    fn update(&mut self, context: &DrawContext, update_interval: &UpdateInterval);
    fn drawables(&self) -> &dyn Drawable;
}
