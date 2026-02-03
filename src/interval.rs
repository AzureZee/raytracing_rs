use crate::Double;

#[derive(Clone, Copy)]
pub struct Interval {
    pub min: Double,
    pub max: Double,
}

impl Interval {
    pub fn new(min: Double, max: Double) -> Self {
        Self { min, max }
    }
    pub fn contains(&self, x: Double) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: Double) -> bool {
        self.min < x && x < self.max
    }
}
