use crate::area;
pub struct Rectangle {
    pub length: f64,
    pub width: f64,
}

impl Rectangle {
    pub fn new(length: f64, width: f64) -> Self {
        Rectangle { length, width }
    }
}

// Implement the Area trait for Rectangle
impl area::Area for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }
}