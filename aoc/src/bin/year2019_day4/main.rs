use separator::usize;

pub fn main() -> anyhow::Result<()> {
    let lower = 256310;
    let upper = 732736;

    let p1 = solve_part_1(lower,upper)?;
    println!("part1 {p1}");
  
    let p2 = solve_part_2(lower,upper)?;
    println!("part2 {p2}");

    Ok(())
}

fn solve_part_1(lower: u32, upper: u32) -> anyhow::Result<usize> {
    Ok((lower..=upper)
        .filter(is_valid_password_part_1)
        .count())
}

fn password_to_digits(password: &u32) -> [u8; 6] {
    [ 
        ((password / 100_000) % 10) as u8,
        ((password / 10_000) % 10) as u8,
        ((password / 1_000) % 10) as u8,
        ((password / 100) % 10) as u8,
        ((password / 10) % 10) as u8,
        (*password % 10) as u8,
    ]
}

fn is_valid_password_part_1(password: &u32) -> bool {
    let digits = password_to_digits(password);

    let mut has_double = false;

    for (n1,n2) in digits.iter().zip(digits[1..].iter()) {
        if n2 < n1 { return false };
        if n1 == n2 { has_double = true }
    }

    has_double
}

fn solve_part_2(lower: u32, upper: u32) -> anyhow::Result<usize> {
    Ok((lower..=upper)
        .filter(is_valid_password_part_2)
        .count())
}

fn is_valid_password_part_2(password: &u32) -> bool {
    let mut digits: &[u8] = &password_to_digits(password);

    for (n1,n2) in digits.iter().zip(digits[1..].iter()) {
        if n2 < n1 { return false };
    }

    while ! digits.is_empty() {
        let digit = digits[0];
        let same = digits.iter().take_while(|n| **n== digit).count();
        if same == 2 { 
            return true; 
        } else {
            digits = &digits[same..];
        }
    } 

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        assert!(is_valid_password_part_1(&111111));
        assert!(!is_valid_password_part_1(&223450));
        assert!(!is_valid_password_part_1(&123789));
    }
    #[test]
    fn test_example_part_2() {
        assert!(is_valid_password_part_2(&112233));
        assert!(!is_valid_password_part_2(&123444));
        assert!(is_valid_password_part_2(&111122));
    }
}
