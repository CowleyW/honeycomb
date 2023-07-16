pub struct CartesianPoint {
    x: f32,
    y: f32,
}

impl CartesianPoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}