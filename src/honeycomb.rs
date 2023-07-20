use crate::cartesian_point::CartesianPoint;
use crate::hex_cell::HexCell;
use std::collections::{HashMap, VecDeque};

pub struct Honeycomb<T> {
    pub grid: Vec<HexCell>,
    pub data: HashMap<HexCell, T>,
    size: usize,
}

impl<T> Honeycomb<T> {
    /// Returns a new `Honeycomb` with the given size.
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

    /// Returns the hex on which the given point is located, or `None` if the point is outside the
    /// bounds of this honeycomb
    pub fn hex_on_point(&self, point: CartesianPoint) -> Option<HexCell> {
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

    /// Returns the shortest path between `from` and `to`, or `None` if no such path can be found.
    ///
    /// Uses a breadth-first approach to finding the path.
    pub fn shortest_path(
        &self,
        from: &HexCell,
        to: &HexCell,
        filter: fn(&T) -> bool,
    ) -> Option<Vec<HexCell>> {
        let mut work_list = VecDeque::<HexCell>::new();
        work_list.push_front(*from);

        let mut came_from = HashMap::<HexCell, HexCell>::new();

        // Loop through the work list as long as we have more cells to check
        while let Some(hex) = work_list.pop_back() {
            for neighbor in self.neighbors_of(hex) {
                // We found our target! Time to reconstruct the path
                if neighbor == *to {
                    let mut path = vec![neighbor, hex];

                    // While we have not returned to the starting cell, append the previous cell to
                    // the path
                    while let Some(v) = came_from.get(path.last().unwrap()) {
                        path.push(*v);

                        if v == from {
                            // path goes from end -> start, we want it the other way around
                            path.reverse();

                            return Some(path);
                        }
                    }
                }

                let seen = came_from.contains_key(&neighbor);

                let valid = if let Some(t) = self.data.get(&neighbor) {
                    filter(t)
                } else {
                    false
                };

                if !seen && valid {
                    came_from.insert(neighbor, hex);
                    work_list.push_front(neighbor);
                }
            }
        }

        None
    }

    /// Returns the valid neighbors of the given hex
    pub fn neighbors_of(&self, hex: HexCell) -> Vec<HexCell> {
        HexCell::directions()
            .into_iter()
            .map(|d| d + hex)
            .filter(|n| n.axial_dist_to(&HexCell::origin()) <= self.size)
            .collect::<Vec<_>>()
    }
}
