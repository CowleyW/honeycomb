use crate::hex_cell::HexCell;

pub struct Honeycomb {
    pub grid: Vec<HexCell>,
}

impl Honeycomb {
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

        Self { grid }
    }
}