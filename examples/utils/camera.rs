use honeycomb::cartesian_point::CartesianPoint;

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub scale: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32, width: f32, height: f32, scale: f32) -> Self {
        Camera {
            x,
            y,
            width,
            height,
            scale,
        }
    }

    pub fn world_to_screen(&self, point: CartesianPoint) -> CartesianPoint {
        let rel_x = point.x - self.x;
        let rel_y = point.y - self.y;

        let screen_x = (rel_x + self.width / 2.0 / 100.0 * self.scale) * 100.0 / self.scale;
        let screen_y = (rel_y + self.height / 2.0 / 100.0 * self.scale) * 100.0 / self.scale;

        CartesianPoint::new(screen_x, screen_y)
    }

    pub fn screen_to_world(&self, x: f32, y: f32) -> CartesianPoint {
        let rel_x = (x * self.scale / 100.0) - self.width / 2.0 / 100.0 * self.scale;
        let rel_y = -(y * self.scale / 100.0) + self.height / 2.0 / 100.0 * self.scale;

        CartesianPoint::new(rel_x + self.x, rel_y + self.y)
    }
}
