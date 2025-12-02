pub fn part2(text: &str) -> anyhow::Result<usize> {
    let mut position: isize = 50;
    let mut score: usize = 0;

    for line in text.lines() {
        let mut num:isize = line[1..].parse()?;  
        let direction: char = line.chars().nth(0).unwrap();

        for _ in 0..num {
            if direction == 'L' {
                position -= 1;
                if position == 0 { score += 1; }
                if position < 0 { position += 100}
            } else if direction == 'R' {
                position += 1;
                if position == 100 { 
                    score += 1; 
                    position = 0;
                }
            }
        }
    }

    Ok(score)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        let example_text = include_str!("example.txt");
        let result = part2(example_text)?;

        assert_eq!(result, 6);   
        Ok(())
    }
}
