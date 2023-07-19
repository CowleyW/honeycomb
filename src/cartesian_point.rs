use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CartesianPoint {
    pub x: f32,
    pub y: f32,
}

impl CartesianPoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Add for CartesianPoint {
    type Output = CartesianPoint;

    fn add(self, rhs: Self) -> Self::Output {
        CartesianPoint::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add for &CartesianPoint {
    type Output = CartesianPoint;

    fn add(self, rhs: Self) -> Self::Output {
        CartesianPoint::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for CartesianPoint {
    type Output = CartesianPoint;

    fn sub(self, rhs: Self) -> Self::Output {
        CartesianPoint::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub for &CartesianPoint {
    type Output = CartesianPoint;

    fn sub(self, rhs: Self) -> Self::Output {
        CartesianPoint::new(self.x - rhs.x, self.y - rhs.y)
    }
}
