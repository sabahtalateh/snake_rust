use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Eq, Copy, Clone, Debug)]
pub struct Coord(u32);

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Coord {
    pub fn new(coord: u32) -> Coord {
        Coord(coord)
    }
}

impl Mul<f64> for Coord {
    type Output = f64;

    fn mul(self, rhs: f64) -> f64 {
        f64::from(self.0) * rhs
    }
}

impl Add<u32> for Coord {
    type Output = Coord;

    fn add(self, rhs: u32) -> Coord {
        Coord(self.0 + rhs)
    }
}

impl Sub<u32> for Coord {
    type Output = Coord;

    fn sub(self, rhs: u32) -> Coord {
        Coord(self.0 - rhs)
    }
}

impl Into<usize> for Coord {
    fn into(self) -> usize {
        self.0 as usize
    }
}
