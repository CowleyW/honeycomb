mod utils;

use crate::utils::camera::Camera;
use honeycomb::hex_cell::HexCell;
use honeycomb::honeycomb::Honeycomb;
use rand::random;
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct Handler {
    honeycomb: Honeycomb<u8>,
    camera: Camera,
}

impl Handler {
    fn new() -> Self {
        let mut honeycomb = Honeycomb::<u8>::new(3);

        for h in honeycomb.grid.iter() {
            honeycomb.data.insert(*h, random::<u8>());
        }

        Self {
            honeycomb,
            camera: Camera::new(0.0, 0.0, WIDTH as f32, HEIGHT as f32, 2.0),
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

        let center = self.camera.world_to_screen(hex.world_location());
        let radius = *self.honeycomb.data.get(hex).unwrap() as f32 / 16.0;
        graphics.draw_circle((center.x, center.y), radius, color);
    }
}

impl WindowHandler for Handler {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.9, 0.9, 0.9));

        for h in &self.honeycomb.grid {
            self.draw_hexagon(h, graphics, Color::BLACK);
        }

        helper.request_redraw();
    }
}

fn main() {
    let window = Window::new_centered("Custom Data", (WIDTH, HEIGHT)).unwrap();

    window.run_loop(Handler::new());
}
