use std::ops::{Range, RangeInclusive};
use std::str::FromStr;

pub(crate) fn line_to_ranges(line: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let a_list : Vec<RangeInclusive<i32>>= line.split(',')
        .map(|l| line_minus(l))
        .collect();

    return (a_list[0].clone(), a_list[1].clone())
}

fn line_minus(line: &str) -> RangeInclusive<i32> {
    let wee: Vec<i32> = line.split('-')
        .map(|v| <i32 as FromStr>::from_str(v).expect("i hate input"))
        .collect();

    return wee[0]..=wee[1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(line_minus("2-4"), 2..=4)
    }

    #[test]
    fn test2() {
        assert_eq!(line_to_ranges("2-4,6-8"), (2..=4, 6..=8));
        assert_eq!(line_to_ranges("2-3,4-5"), (2..=3, 4..=5));
    }
}