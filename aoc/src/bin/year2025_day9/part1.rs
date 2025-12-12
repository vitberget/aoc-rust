use aoc_procmacros::aoc_profile;

pub struct Coordinate {
    pub x: usize,
    pub y: usize
}

impl From<&str> for Coordinate {
    fn from(line: &str) -> Self {
        let mut iterator = line.split(",");
        let x: usize = iterator.next().unwrap().parse().unwrap();
        let y: usize = iterator.next().unwrap().parse().unwrap();
        Self { x, y }
    }
}

impl Coordinate {
    pub fn parse_text(text: &str) -> Vec<Coordinate> {
        text.lines()
            .map(Self::from)
            .collect()
    }

}

#[aoc_profile]
pub fn part1(text: &str) -> anyhow::Result<usize> {
    let coordinates: Vec<Coordinate> = Coordinate::parse_text(text);

    let mut size = 0;

    for i in 0..coordinates.len() {
        let coordinate_1 = &coordinates[i];
        for coordinate_2 in coordinates.iter().skip(i+1) {
            let dx = usize::max(coordinate_1.x, coordinate_2.x) - usize::min(coordinate_1.x, coordinate_2.x) + 1;
            let dy = usize::max(coordinate_1.y, coordinate_2.y) - usize::min(coordinate_1.y, coordinate_2.y) + 1;
          
            size = usize::max(size, dx*dy);
        }
    }

    Ok(size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        const EXAMPLE: &str = include_str!("example.txt");
        let result = part1(EXAMPLE)?;
        assert_eq!(result, 50);
        Ok(())
    }
}
