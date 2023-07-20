use crate::cartesian_point::CartesianPoint;
use crate::hex_cell::HexCell;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

pub struct Honeycomb<T> {
    pub grid: Vec<HexCell>,
    pub data: HashMap<HexCell, T>,
    size: usize,
}

impl<T> Honeycomb<T> {
    pub fn new(size: usize) -> Self {
        let n = size as i32;
        let mut grid = Vec::new();

        for q in -n..=n {
            let r1 = std::cmp::max(-n, -q - n);
            let r2 = std::cmp::min(n, -q + n);

            for r in r1..=r2 {
                grid.push(HexCell::new(q, r));
            }
        }
        let data = HashMap::new();

        Self { grid, data, size }
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

    pub fn shortest_path(
        &self,
        from: &HexCell,
        to: &HexCell,
        filter: fn(&T) -> bool,
    ) -> Option<Vec<HexCell>> {
        let mut queue = VecDeque::<HexCell>::new();
        queue.push_front(*from);

        let mut came_from = HashMap::<HexCell, HexCell>::new();

        while let Some(h) = queue.pop_back() {
            if h == *to {
                let mut path = vec![h];
                while let Some(v) = came_from.get(path.last().unwrap()) {
                    path.push(*v);

                    if v == from {
                        path.reverse();
                        return Some(path);
                    }
                }
            } else {
                for n in self.neighbors_of(h) {
                    let seen = came_from.contains_key(&n);

                    let valid = if let Some(t) = self.data.get(&n) {
                        filter(t)
                    } else {
                        false
                    };

                    if !seen && valid {
                        came_from.insert(n, h);
                        queue.push_front(n);
                    }
                }
            }
        }

        None
    }

    pub fn neighbors_of(&self, hex: HexCell) -> Vec<HexCell> {
        let directions: [HexCell; 6] = [
            HexCell::new(1, 0),
            HexCell::new(1, -1),
            HexCell::new(0, -1),
            HexCell::new(-1, 0),
            HexCell::new(-1, 1),
            HexCell::new(0, 1),
        ];

        directions
            .into_iter()
            .map(|d| d + hex)
            .filter(|n| n.axial_dist_to(&HexCell::origin()) <= self.size)
            .collect::<Vec<_>>()
    }
}
