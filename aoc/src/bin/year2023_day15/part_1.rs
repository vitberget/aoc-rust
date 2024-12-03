pub fn solve_part_1(text: &str) -> anyhow::Result<u32> {
    Ok(text.split(',')
           .map(hash)
           .map(|byte| byte as u32)
           .sum())
}

pub(crate) fn hash(part: &str) -> u8 {
    part.chars()
        .map(|ch| ch as u8)
        .fold(0u8, |acc, ch| acc.overflowing_add(ch).0.overflowing_mul(17).0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashes() {
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("qp=3"), 97);
        assert_eq!(hash("cm=2"), 47);
        assert_eq!(hash("qp-"), 14);
        assert_eq!(hash("pc=4"), 180);
        assert_eq!(hash("ot=9"), 9);
        assert_eq!(hash("ab=5"), 197);
        assert_eq!(hash("pc-"), 48);
        assert_eq!(hash("pc=6"), 214);
        assert_eq!(hash("ot=7"), 231);
    }

    #[test]
    fn test_example() {
        assert_eq!(solve_part_1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7").unwrap(), 1320);
    }
}
