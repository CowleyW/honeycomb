use crate::cartesian_point::CartesianPoint;
use crate::hex_cell::HexCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};

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
    pub fn world_to_hex(&self, point: CartesianPoint) -> Option<HexCell> {
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
    ///
    /// [`filter`] takes in the current cell's value and the new cell's value, and returns whether
    /// it is legal to move across this cell
    pub fn shortest_path(
        &self,
        from: &HexCell,
        to: &HexCell,
        filter: fn(curr_val: &T, new_val: &T) -> bool,
    ) -> Option<Vec<HexCell>> {
        let mut work_list = VecDeque::<HexCell>::new();
        work_list.push_front(*from);

        let mut came_from = HashMap::<HexCell, HexCell>::new();

        // https://en.wikipedia.org/wiki/Breadth-first_search
        while let Some(hex) = work_list.pop_back() {
            for neighbor in self.neighbors_of(hex) {
                // We found our target! Time to reconstruct the path
                if neighbor == *to {
                    return Self::reconstruct_path(*to, hex, *from, came_from);
                }

                let seen = came_from.contains_key(&neighbor);

                let valid = if let (Some(curr), Some(new)) = (self.data.get(&hex), self.data.get(&neighbor)) {
                    filter(curr, new)
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

    /// Calculates the cheapest path between `from` and `to` using the given cost function and
    /// heuristic.
    ///
    /// [`cost_fn`] takes in the current cell's value and the new cell's value, and returns a usize
    /// cost for movement
    ///
    /// [`heuristic`] takes in the current cell and the destination and cell, and returns a usize
    /// estimate for the distance to the destination. The speed of this function is heavily affected
    /// by the given heuristic function; the more strict the function is, the less additional paths
    /// will be explored. However, this function is ONLY guaranteed to find the cheapest path if the
    /// heuristic function NEVER overestimates the cost of reaching the destination.
    pub fn cheapest_path( 
        &self,
        from: &HexCell,
        to: &HexCell,
        cost_fn: fn(curr_val: &T, next_val: &T) -> usize,
        heuristic: fn(curr: &HexCell, dest: &HexCell) -> usize,
    ) -> Option<Vec<HexCell>> {
        let mut work_list = BinaryHeap::<HexWeight>::new();
        work_list.push(HexWeight::new(0, *from));

        let mut came_from = HashMap::<HexCell, HexCell>::new();
        let mut cost_so_far = HashMap::<HexCell, usize>::new();
        cost_so_far.insert(*from, 0);

        // https://en.wikipedia.org/wiki/A*_search_algorithm
        while let Some(weight) = work_list.pop() {
            let curr = weight.hex;

            for neighbor in self.neighbors_of(curr) {
                if &neighbor == to {
                    return Self::reconstruct_path(*to, curr, *from, came_from);
                }

                if let (Some(old_val), Some(new_val)) = (self.data.get(&curr), self.data.get(&neighbor)) {
                    let new_cost = cost_so_far.get(&curr).unwrap() + cost_fn(old_val, new_val);

                    let is_cheaper = if let Some(cost) = cost_so_far.get(&neighbor) {
                        new_cost < *cost
                    } else {
                        true
                    };

                    if is_cheaper {
                        cost_so_far.insert(neighbor, new_cost);

                        let priority = new_cost + heuristic(&neighbor, to);
                        work_list.push(HexWeight::new(priority, neighbor));

                        came_from.insert(neighbor, curr);
                    }
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

    /// Reconstructs the path from the destination to the start given a map from cell to cell
    fn reconstruct_path(dest: HexCell, penultimate: HexCell, start: HexCell, came_from: HashMap<HexCell, HexCell>) -> Option<Vec<HexCell>> {
        let mut path = vec![dest, penultimate];

        // While we have not returned to the starting cell, append the previous cell to
        // the path
        while let Some(v) = came_from.get(path.last().unwrap()) {
            path.push(*v);

            if v == &start {
                // path goes from end -> start, we want it the other way around
                path.reverse();

                return Some(path);
            }
        }

        None
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HexWeight {
    pub cost: usize,
    pub hex: HexCell,
}

impl HexWeight {
    fn new(cost: usize, hex: HexCell) -> Self {
        Self { cost, hex }
    }
}

impl PartialOrd<Self> for HexWeight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HexWeight {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
