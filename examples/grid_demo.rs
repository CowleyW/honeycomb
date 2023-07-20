mod utils;

use crate::utils::camera::Camera;
use honeycomb::hex_cell::HexCell;
use honeycomb::honeycomb::Honeycomb;
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct Handler {
    honeycomb: Honeycomb<u8>,
    camera: Camera,
    target_hex: Option<HexCell>,
}

impl Handler {
    fn new() -> Self {
        Self {
            honeycomb: Honeycomb::new(3),
            camera: Camera::new(0.0, 0.0, WIDTH as f32, HEIGHT as f32, 2.0),
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

        for h in &self.honeycomb.grid {
            self.draw_hexagon(h, graphics, Color::BLACK);
        }

        if let Some(h) = self.target_hex {
            let color = match h.axial_dist_to(&HexCell::origin()) {
                0 => Color::RED,
                1 => Color::GREEN,
                2 => Color::BLUE,
                _ => Color::YELLOW,
            };

            self.draw_hexagon(&h, graphics, color);
        }

        helper.request_redraw();
    }

    fn on_mouse_move(&mut self, _helper: &mut WindowHelper<()>, position: Vec2) {
        let world_position = self.camera.screen_to_world(position.x, position.y);
        self.target_hex = self.honeycomb.hex_on_point(world_position);
    }
}

fn main() {
    let window = Window::new_centered("Honeycomb Grid", (WIDTH, HEIGHT)).unwrap();

    window.run_loop(Handler::new());
}
