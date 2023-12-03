use std::collections::HashSet;

#[derive(Debug)]
pub struct Schematic {
    pub numbers: Vec<Number>,
    pub symbols: HashSet<Position>
}

#[derive(Debug)]
pub struct Number {
    pub number: u32,
    pub positions: HashSet<Position>
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y:usize) -> Self {
        Self {x, y}
    }

    pub fn surrounding(&self) -> HashSet<Self> {
        let mut result: HashSet<Self> = HashSet::new();
    
        result.insert(Position::new(self.x - 1, self.y - 1));
        result.insert(Position::new(self.x - 1, self.y));
        result.insert(Position::new(self.x - 1, self.y + 1));
        result.insert(Position::new(self.x, self.y - 1));
        result.insert(Position::new(self.x, self.y + 1));
        result.insert(Position::new(self.x + 1, self.y - 1));
        result.insert(Position::new(self.x + 1, self.y));
        result.insert(Position::new(self.x + 1, self.y + 1));

        result
    }
}
