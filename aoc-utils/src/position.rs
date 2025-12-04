use std::collections::HashSet;
use std::ops::{Add, Mul, MulAssign, Sub};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<isize> for Position {
    type Output = Self;

    fn mul(self, other: isize) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other
        }
    }
}

impl MulAssign<isize> for Position {
    fn mul_assign(&mut self, rhs: isize) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Position {
    pub fn new(x:isize, y:isize) -> Self { Self {x, y} }
    pub fn manhattan(&self) -> isize { self.x.abs() + self.y.abs() }
    pub fn get_surrounding(&self) -> HashSet<Position> {
        let mut result: HashSet<Position> = HashSet::new();
        for x in -1..=1 {
            for y in -1..=1 {
                if x != 0 || y != 0 {
                   result.insert(*self + Position { x, y });
                }
            }
        }
        result
    }
}
