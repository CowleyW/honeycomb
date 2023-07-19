use honeycomb::cartesian_point::CartesianPoint;
use honeycomb::hex_cell::HexCell;
use honeycomb::honeycomb::Honeycomb;
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct Camera {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
}

impl Camera {
    fn new(x: f32, y: f32, scale: f32) -> Self {
        Camera { x, y, scale }
    }

    fn world_to_screen(&self, point: CartesianPoint) -> CartesianPoint {
        let rel_x = point.x - self.x;
        let rel_y = point.y - self.y;

        let screen_x = (rel_x + (WIDTH as f32) / 2.0 / 100.0 * self.scale) * 100.0 / self.scale;
        let screen_y = (rel_y + (HEIGHT as f32) / 2.0 / 100.0 * self.scale) * 100.0 / self.scale;

        CartesianPoint::new(screen_x, screen_y)
    }

    fn screen_to_world(&self, x: f32, y: f32) -> CartesianPoint {
        let rel_x = (x * self.scale / 100.0) - (WIDTH as f32) / 2.0 / 100.0 * self.scale;
        let rel_y = -(y * self.scale / 100.0) + (HEIGHT as f32) / 2.0 / 100.0 * self.scale;

        CartesianPoint::new(rel_x + self.x, rel_y + self.y)
    }
}

struct Handler {
    grid: Honeycomb,
    camera: Camera,
    target_hex: Option<HexCell>,
}

impl Handler {
    fn new() -> Self {
        Self {
            grid: Honeycomb::new(3),
            camera: Camera::new(0.0, 0.0, 2.0),
            target_hex: None,
        }
    }

    fn draw_hexagon(&self, hex: &HexCell, graphics: &mut Graphics2D, color: Color) {
        let vertices = hex
            .vertex_locations()
            .into_iter()
            .map(|p| self.camera.world_to_screen(p))
            .map(|p| (p.x, p.y))
            .collect::<Vec<_>>();

        graphics.draw_line(vertices[0], vertices[1], 4.0, color);
        graphics.draw_line(vertices[1], vertices[2], 4.0, color);
        graphics.draw_line(vertices[2], vertices[3], 4.0, color);
        graphics.draw_line(vertices[3], vertices[4], 4.0, color);
        graphics.draw_line(vertices[4], vertices[5], 4.0, color);
        graphics.draw_line(vertices[5], vertices[0], 4.0, color);
    }
}

impl WindowHandler for Handler {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.9, 0.9, 0.9));

        for h in &self.grid.grid {
            self.draw_hexagon(h, graphics, Color::BLACK);
        }

        if let Some(h) = self.target_hex {
            self.draw_hexagon(&h, graphics, Color::RED);
        }

        helper.request_redraw();
    }

    fn on_mouse_move(&mut self, _helper: &mut WindowHelper<()>, position: Vec2) {
        let world_position = self.camera.screen_to_world(position.x, position.y);
        self.target_hex = self.grid.nearest_hex(world_position);
    }
}

fn main() {
    let window = Window::new_centered("Honeycomb Grid", (800, 600)).unwrap();

    window.run_loop(Handler::new());
}
