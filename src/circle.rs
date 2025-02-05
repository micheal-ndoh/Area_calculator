use crate::area;
use std::f64::consts::PI;

pub struct Circle {
    pub radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Circle { radius }
    }
}

// Implement the Area trait for Circle
impl area::Area for Circle {
    fn area(&self) -> f64 {
        // 3.14 * self.radius * self.radius
      PI * self.radius * self.radius
    }
}