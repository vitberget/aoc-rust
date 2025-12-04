use aoc_procmacros::aoc_profile;

#[aoc_profile]
pub fn part1(text: &str) -> anyhow::Result<usize> {
    Ok(text.trim().lines()
        .filter(|line| is_valid_triangle(line))
        .count())
}

pub fn is_valid_triangle(line: &str) -> bool {
    let mut iter = line
        .split_whitespace()
        .flat_map(|word| word.parse::<usize>());

    let n1 = iter.next();
    let n2 = iter.next();
    let n3 = iter.next();

    if let Some(n1) = n1 && let Some(n2) = n2 && let Some(n3) = n3 {
            n1 + n2 > n3 &&
            n1 + n3 > n2 &&
            n2 + n3 > n1
    } else {
        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_triangle() {
        assert!(! is_valid_triangle("5 10 25"));
    }
}
