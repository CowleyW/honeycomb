mod utils;

use crate::utils::camera::Camera;
use honeycomb::hex_cell::HexCell;
use honeycomb::honeycomb::Honeycomb;
use itertools::Itertools;
use rand::random;
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::shape::Polygon;
use speedy2d::window::{KeyScancode, VirtualKeyCode, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

const WIDTH: u32 = 1600;
const HEIGHT: u32 = 1200;

struct State {
    honeycomb: Honeycomb<u8>,
    camera: Camera,
    last_mouse_position: Vec2,
    start_hex: Option<HexCell>,
    end_hex: Option<HexCell>,
}

impl State {
    fn new() -> Self {
        let mut honeycomb = Honeycomb::<u8>::new(20);

        for h in honeycomb.grid.iter() {
            let val = random::<u8>() % 4 * 85;
            honeycomb.data.insert(*h, val);
        }

        Self {
            honeycomb,
            camera: Camera::new(0.0, 0.0, WIDTH as f32, HEIGHT as f32, 5.2),
            last_mouse_position: Vec2::new(-1.0, -1.0),
            start_hex: None,
            end_hex: None,
        }
    }

    fn draw_hexagon(&self, hex: &HexCell, graphics: &mut Graphics2D, color: Color) {
        let cos30 = 3f32.sqrt() / 2.0;

        let vertices = [
            (0f32, 1f32),
            (cos30, 0.5f32),
            (cos30, -0.5f32),
            (0.0f32, -1.0f32),
            (-cos30, -0.5f32),
            (-cos30, 0.5f32),
        ]
        .map(|(x, y)| (x * 100.0 / self.camera.scale, y * 100.0 / self.camera.scale));
        let polygon = Polygon::new(&vertices);
        let o = self.camera.world_to_screen(hex.world_location());

        graphics.draw_polygon(&polygon, (o.x, o.y), color);
    }

    fn draw_line(&self, from: &HexCell, to: &HexCell, graphics: &mut Graphics2D, color: Color) {
        let screen_from = self.camera.world_to_screen(from.world_location());
        let screen_to = self.camera.world_to_screen(to.world_location());

        graphics.draw_line(
            (screen_from.x, screen_from.y),
            (screen_to.x, screen_to.y),
            4.0,
            color,
        );
    }
}

impl WindowHandler for State {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.9, 0.9, 0.9));

        for h in &self.honeycomb.grid {
            let intensity = 255 - *self.honeycomb.data.get(h).unwrap();
            let color = Color::from_int_rgb(intensity, intensity, intensity);
            self.draw_hexagon(h, graphics, color);
        }

        if let (Some(start), Some(finish)) = (self.start_hex, self.end_hex) {
            let path = self
                .honeycomb
                .cheapest_path(&start, &finish, |a, b| a.axial_dist_to(b))
                .unwrap();

            for (h1, h2) in path.iter().tuple_windows() {
                self.draw_line(h1, h2, graphics, Color::GREEN);
            }
        }

        helper.request_redraw();
    }

    fn on_mouse_move(&mut self, _helper: &mut WindowHelper<()>, position: Vec2) {
        self.last_mouse_position = position;
    }

    fn on_key_down(
        &mut self,
        _helper: &mut WindowHelper<()>,
        virtual_key_code: Option<VirtualKeyCode>,
        _scancode: KeyScancode,
    ) {
        if let Some(keycode) = virtual_key_code {
            let nearest_hex = self.honeycomb.hex_on_point(
                self.camera
                    .screen_to_world(self.last_mouse_position.x, self.last_mouse_position.y),
            );

            match (keycode, self.start_hex, self.end_hex) {
                (VirtualKeyCode::Space, Some(_), None) => self.end_hex = nearest_hex,
                (VirtualKeyCode::Space, None, _) => self.start_hex = nearest_hex,
                (VirtualKeyCode::Delete, Some(_), _) => self.start_hex = None,
                (VirtualKeyCode::Delete, None, Some(_)) => self.end_hex = None,
                (_, _, _) => (),
            }
        }
    }
}

fn main() {
    let window = Window::new_centered("Custom Data", (WIDTH, HEIGHT)).unwrap();

    window.run_loop(State::new());
}
