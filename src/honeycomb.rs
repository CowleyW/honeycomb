use crate::cartesian_point::CartesianPoint;
use crate::hex_cell::HexCell;
use std::collections::HashMap;

pub struct Honeycomb<T> {
    pub grid: Vec<HexCell>,
    pub data: HashMap<HexCell, T>,
}

impl<T> Honeycomb<T> {
    pub fn new(n: usize) -> Self {
        let n = n as i32;
        let mut grid = Vec::new();

        for q in -n..=n {
            let r1 = std::cmp::max(-n, -q - n);
            let r2 = std::cmp::min(n, -q + n);

            for r in r1..=r2 {
                grid.push(HexCell::new(q, r));
            }
        }
        let data = HashMap::new();

        Self { grid, data }
    }

    pub fn nearest_hex(&self, point: CartesianPoint) -> Option<HexCell> {
        let frac_q = 3f32.sqrt() / 3.0 * point.x - 1.0 / 3.0 * point.y;
        let frac_r = 2.0 / 3.0 * point.y;
        let frac_s = -frac_q - frac_r;

        let q = frac_q.round();
        let r = frac_r.round();
        let s = frac_s.round();

        let dq = (q - frac_q).abs();
        let dr = (r - frac_r).abs();
        let ds = (s - frac_s).abs();

        let cell = if dq > dr && dq > ds {
            HexCell::new((-r - s) as i32, r as i32)
        } else if dr > ds {
            HexCell::new(q as i32, (-q - s) as i32)
        } else {
            HexCell::new(q as i32, r as i32)
        };

        if self.grid.contains(&cell) {
            Some(cell)
        } else {
            None
        }
    }
}
