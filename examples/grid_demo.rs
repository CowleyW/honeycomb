use speedy2d::{Graphics2D, Window};
use speedy2d::color::Color;
use speedy2d::shape::Polygon;
use speedy2d::window::{WindowHandler, WindowHelper};
use honeycomb::honeycomb::Honeycomb;

struct Camera {
    pub x: f32,
    pub y: f32,
}

impl Camera {
    fn new(x: f32, y: f32) -> Self {
        Camera { x, y }
    }
}

struct Handler {
    grid: Honeycomb,
    camera: Camera
}

impl Handler {
    fn new() -> Self {
        Self {
            grid: Honeycomb::new(3),
            camera: Camera::new(0.0, 0.0),
        }
    }

    fn draw_hexagon(&self, x: f32, y: f32, graphics: &mut Graphics2D) {
        let vertices = [
            (0.0 + x, 50.0 + y), (43.0 + x, 25.0 + y), (43.0 + x, -25.0 + y),
            (0.0 + x, -50.0 + y), (-43.0 + x, -25.0 + y), (-43.0 + x, 25.0 + y),
        ];

        graphics.draw_line(vertices[0], vertices[1], 4.0, Color::BLACK);
        graphics.draw_line(vertices[1], vertices[2], 4.0, Color::BLACK);
        graphics.draw_line(vertices[2], vertices[3], 4.0, Color::BLACK);
        graphics.draw_line(vertices[3], vertices[4], 4.0, Color::BLACK);
        graphics.draw_line(vertices[4], vertices[5], 4.0, Color::BLACK);
        graphics.draw_line(vertices[5], vertices[0], 4.0, Color::BLACK);
    }
}

impl WindowHandler for Handler {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.9, 0.9, 0.9));

        for h in &self.grid.grid {
            let rel_x = h.cartesian_point().x - self.camera.x;
            let rel_y = h.cartesian_point().y - self.camera.y;

            let screen_x = (rel_x + 8.0) * 50.0;
            let screen_y = 600.0 - (rel_y + 6.0) * 50.0;

            self.draw_hexagon(screen_x, screen_y, graphics);
        }

        helper.request_redraw();
    }
}

fn main() {
    let window = Window::new_centered("Honeycomb Grid", (800, 600)).unwrap();

    window.run_loop(Handler::new());
}