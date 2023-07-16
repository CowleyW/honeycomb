use std::ops::{Add, Sub};
use crate::cartesian_point::CartesianPoint;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HexCell {
    q: i32,
    r: i32,
}

impl HexCell {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    pub fn neighbors(&self) -> Vec<HexCell> {
        let directions: [HexCell; 6] = [
            HexCell::new(1, 0), HexCell::new(1, -1),
            HexCell::new(0, -1), HexCell::new(-1, 0),
            HexCell::new(-1, 1), HexCell::new(0, 1),
        ];

        directions.into_iter().map(|d| &d + self).collect::<Vec<_>>()
    }

    pub fn cartesian_pos(&self) -> CartesianPoint {
        let y = -1.5 * r as f32;
        let x = (3 as f32).sqrt() * 0.5 * (r as f32 / 2.0 + q as f32);

        CartesianPoint::new(x, y)
    }
}

impl Add for HexCell {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
        }
    }
}

impl Add for &HexCell {
    type Output = HexCell;

    fn add(self, rhs: Self) -> Self::Output {
        HexCell {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
        }
    }
}

impl Sub for HexCell {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
        }
    }
}

impl Sub for &HexCell {
    type Output = HexCell;

    fn sub(self, rhs: Self) -> Self::Output {
        HexCell {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hex_cell::HexCell;

    #[test]
    fn test_add() {
        let h1 = HexCell::new(10, 10);
        let h2 = HexCell::new(5, -5);

        assert_eq!(h1, HexCell::new(10, 10));
        assert_eq!(h2, HexCell::new(5, -5));

        let h3 = h1 + h2;

        assert_eq!(h3, HexCell::new(15, 5));
    }

    #[test]
    fn test_sub() {
        let h1 = HexCell::new(10, 10);
        let h2 = HexCell::new(5, -5);

        assert_eq!(h1, HexCell::new(10, 10));
        assert_eq!(h2, HexCell::new(5, -5));

        let h3 = h1 - h2;

        assert_eq!(h3, HexCell::new(5, 15));
    }

    #[test]
    fn test_neighbors() {
        let h = HexCell::new(5, 5);
        let neighbors = h.neighbors();

        assert_eq!(neighbors.len(), 6);

        assert!(neighbors.contains(&HexCell::new(6, 5)));
        assert!(neighbors.contains(&HexCell::new(6, 4)));
        assert!(neighbors.contains(&HexCell::new(5, 4)));
        assert!(neighbors.contains(&HexCell::new(4, 5)));
        assert!(neighbors.contains(&HexCell::new(4, 6)));
        assert!(neighbors.contains(&HexCell::new(5, 6)));
    }

    #[test]
    fn test_cartesian_pos() {
        
    }
}