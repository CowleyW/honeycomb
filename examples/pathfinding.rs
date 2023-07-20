mod utils;

use crate::utils::camera::Camera;
use honeycomb::hex_cell::HexCell;
use honeycomb::honeycomb::Honeycomb;
use rand::random;
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::window::{KeyScancode, VirtualKeyCode, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 900;

struct State {
    honeycomb: Honeycomb<bool>,
    camera: Camera,
    last_mouse_position: Vec2,
    start_hex: Option<HexCell>,
    end_hex: Option<HexCell>,
}

impl State {
    fn new() -> Self {
        let mut honeycomb = Honeycomb::<bool>::new(20);

        for h in honeycomb.grid.iter() {
            honeycomb.data.insert(*h, random::<bool>());
        }

        Self {
            honeycomb,
            camera: Camera::new(0.0, 0.0, WIDTH as f32, HEIGHT as f32, 8.0),
            last_mouse_position: Vec2::new(-1.0, -1.0),
            start_hex: None,
            end_hex: None,
        }
    }

    fn draw_hexagon(&self, hex: &HexCell, graphics: &mut Graphics2D, color: Color) {
        let vertices = hex
            .vertex_locations()
            .into_iter()
            .map(|p| self.camera.world_to_screen(p))
            .map(|p| (p.x, p.y))
            .collect::<Vec<_>>();

        graphics.draw_line(vertices[0], vertices[1], 2.0, color);
        graphics.draw_line(vertices[1], vertices[2], 2.0, color);
        graphics.draw_line(vertices[2], vertices[3], 2.0, color);
        graphics.draw_line(vertices[3], vertices[4], 2.0, color);
        graphics.draw_line(vertices[4], vertices[5], 2.0, color);
        graphics.draw_line(vertices[5], vertices[0], 2.0, color);
    }
}

impl WindowHandler for State {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.9, 0.9, 0.9));

        for h in &self.honeycomb.grid {
            self.draw_hexagon(h, graphics, Color::BLACK);
            let walkable = self.honeycomb.data.get(h).unwrap();

            if *walkable {
            } else {
                self.draw_hexagon(h, graphics, Color::RED);
            }
        }

        let ground = self
            .honeycomb
            .grid
            .iter()
            .filter(|h| {
                if let Some(d) = self.honeycomb.data.get(*h) {
                    *d
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();

        let walls = self
            .honeycomb
            .grid
            .iter()
            .filter(|h| !ground.contains(h))
            .collect::<Vec<_>>();

        for h in ground {
            self.draw_hexagon(h, graphics, Color::LIGHT_GRAY);
        }

        for h in walls {
            self.draw_hexagon(h, graphics, Color::BLACK);
        }

        if let Some(a) = self.start_hex {
            self.draw_hexagon(&a, graphics, Color::GREEN);
        }

        if let Some(b) = self.end_hex {
            self.draw_hexagon(&b, graphics, Color::RED);
        }

        if let (Some(a), Some(b)) = (self.start_hex, self.end_hex) {
            let p = self.honeycomb.shortest_path(&a, &b, |x| *x);

            if let Some(path) = p {
                for h in path {
                    self.draw_hexagon(&h, graphics, Color::CYAN);
                }
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
            let nearest_hex = self.honeycomb.nearest_hex(
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
