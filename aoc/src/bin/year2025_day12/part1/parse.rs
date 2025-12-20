use crate::part1::structs::shape::Shape;
use crate::part1::structs::requirements::Requirements;

pub fn parse_input(text: &str) -> anyhow::Result<(Vec<Shape>, Vec<Requirements>)> {
    let mut shapes: Vec<Shape> = vec![];
    let mut requirements: Vec<Requirements> = vec![];

    let mut shape_lines: Vec<&str> = vec![];

    for line in text.lines() {
        if line.ends_with(":") { shape_lines = vec![] }
        else if line.is_empty() { shapes.push(Shape::parse(&shape_lines));}
        else if line.contains("x") { requirements.push(line.try_into()?); }
        else { shape_lines.push(line); }
    }

    Ok((shapes, requirements))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_example() -> anyhow::Result<()> {
        let (shapes, requirements) = parse_input(include_str!("../example.txt"))?;
        assert_eq!(shapes.len(), 6);
        assert_eq!(requirements.len(), 3);
        Ok(())
    }
}
