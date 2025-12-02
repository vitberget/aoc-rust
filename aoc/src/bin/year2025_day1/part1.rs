pub fn part1(text: &str) -> anyhow::Result<usize> {
    let mut position: isize = 50;
    let mut score: usize = 0;

    for line in text.lines() {
        let num:isize = line[1..].parse()?;  

        let direction: char = line.chars().nth(0).unwrap();

        if direction == 'L' {
            position -= num;
        } else if direction == 'R' {
            position += num;
        }
        position %= 100;
        if position < 0 { position += 100; }

        if position == 0 { score += 1; }
    }

    Ok(score)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        let example_text = include_str!("example.txt");

        let result = part1(example_text)?;

        assert_eq!(result, 3);   
        Ok(())
    }
}
