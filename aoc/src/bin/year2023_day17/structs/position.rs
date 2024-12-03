use std::fmt::Display;
use std::ops::{Sub, Add};

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub(crate) struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x:i32, y:i32) -> Self {
        Self { x, y }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{},{}]", self.x, self.y))
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_add() {
        use Position as P;
        assert_eq!(P::new(0,0) + P::new(0,0), P::new(0,0));
        assert_eq!(P::new(2,2) + P::new(0,0), P::new(2,2));
        assert_eq!(P::new(2,2) + P::new(0,1), P::new(2,3));
        assert_eq!(P::new(2,2) + P::new(1,0), P::new(3,2));
        assert_eq!(P::new(2,2) + P::new(0,-1), P::new(2,1));
        assert_eq!(P::new(2,2) + P::new(-1,0), P::new(1,2));
    }
}
