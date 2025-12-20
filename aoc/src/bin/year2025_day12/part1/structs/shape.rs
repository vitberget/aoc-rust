use std::collections::HashSet;
use std::hash::Hash;

use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Shape {
    pub pixels: HashSet<(i64,i64)>
}

impl Hash for Shape {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pixels
            .iter()
            .sorted()
            .collect_vec()
            .hash(state);
    }
}

impl Shape {
    pub fn parse(lines: &Vec<&str>) -> Self {
        let pixels = lines.iter().enumerate()
            .flat_map(|(y, line)| line.chars().enumerate()
                .map(move |(x,ch)| (x,y,ch)))
            .filter(|(_,_,ch)| *ch == '#')
            .map(|(x,y,_)| (x as i64, y as i64))
            .map(|(x,y)| (x - 1, y - 1))
            .collect();
        Self { pixels }
    }
    
    pub fn rotate(&self) -> Self {
        let pixels = self.pixels.iter()
            .map(|(x,y)| (-*y, *x))
            .collect();
        Self { pixels }
    }

    pub fn mirror_x(&self) -> Self {
        let pixels = self.pixels.iter()
            .map(|(x,y)| (-*x, *y))
            .collect();
        Self { pixels }
    }
    pub fn mirror_y(&self) -> Self {
        let pixels = self.pixels.iter()
            .map(|(x,y)| (*x, -*y))
            .collect();
        Self { pixels }
    }

    pub fn variations(&self) -> Vec<Self> {
        let mut variations: HashSet<Self> = HashSet::new();

        variations.insert(self.to_owned());
        variations.insert(self.mirror_x());
        variations.insert(self.mirror_y());
        variations.insert(self.mirror_x().mirror_y());

        for _ in 0..4 {
            let rotated = self.rotate();
            variations.insert(rotated.mirror_x());
            variations.insert(rotated.mirror_y());
            variations.insert(rotated.mirror_x().mirror_y());
            variations.insert(rotated);
        }

        variations.into_iter().collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_1() {
        let lines = vec![
            "#..", 
            "...", 
            "..."];
        let shape = Shape::parse(&lines);
        assert_eq!(shape.pixels.len(), 1);
        assert!(shape.pixels.contains(&(-1,-1)));
    }

    #[test]
    fn test_rotate_1() {
        let lines = vec![
            "#..", 
            "...", 
            "..."];
        let shape_before = Shape::parse(&lines);
        let lines = vec![
            "..#", 
            "...", 
            "..."];
        let shape_facit = Shape::parse(&lines);

        assert_eq!(shape_before.rotate(), shape_facit)
    }

    #[test]
    fn test_rotate_2() {
        let lines = vec![
            "#..", 
            "...", 
            "..."];
        let shape_before = Shape::parse(&lines);
        let lines = vec![
            "...", 
            "...", 
            "..#"];
        let shape_facit = Shape::parse(&lines);

        assert_eq!(shape_before.rotate().rotate(), shape_facit)
    }

    #[test]
    fn test_rotate_3() {
        let lines = vec![
            "#..", 
            "...", 
            "..."];
        let shape_before = Shape::parse(&lines);
        let lines = vec![
            "...", 
            "...", 
            "#.."];
        let shape_facit = Shape::parse(&lines);

        assert_eq!(shape_before.rotate().rotate().rotate(), shape_facit)
    }

    #[test]
    fn test_variations_1a() {
        let lines = vec![
            "#..", 
            "...", 
            "..."];
        let original = Shape::parse(&lines);
        let variations = original.variations();
        assert_eq!(variations.len(), 4);
        assert!(variations.contains(&Shape::parse(&vec!["#..", "...", "..."])));
        assert!(variations.contains(&Shape::parse(&vec!["..#", "...", "..."])));
        assert!(variations.contains(&Shape::parse(&vec!["...", "...", "#.."])));
        assert!(variations.contains(&Shape::parse(&vec!["...", "...", "..#"])));
    }

    #[test]
    fn test_variations_1b() {
        let lines = vec![
            ".#.", 
            "...", 
            "..."];
        let original = Shape::parse(&lines);
        let variations = original.variations();
        assert_eq!(variations.len(), 4);
        assert!(variations.contains(&Shape::parse(&vec![".#.", "...", "..."])));
        assert!(variations.contains(&Shape::parse(&vec!["...", "#..", "..."])));
        assert!(variations.contains(&Shape::parse(&vec!["...", "..#", "..."])));
        assert!(variations.contains(&Shape::parse(&vec!["...", "...", ".#."])));
    }
}
