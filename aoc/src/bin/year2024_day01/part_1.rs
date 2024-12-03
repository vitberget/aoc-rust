pub fn solve_part_1(numbers: &(Vec<i64>, Vec<i64>)) -> i64 {
    let (mut left, mut right) = numbers.clone();

    left.sort();
    right.sort();

    left.iter().zip(right.iter())
        .map(|(l,r)| l - r)
        .map(i64::abs)
        .sum()
}
