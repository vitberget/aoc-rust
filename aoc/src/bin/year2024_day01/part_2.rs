pub fn solve_part_2(numbers: &(Vec<i64>, Vec<i64>)) -> i64 {
    let (left, right) = numbers;

    left.iter()
        .map(|l| l * count_in_vec(l, right))
        .sum()
}

fn count_in_vec(num: &i64, numbers: &[i64]) -> i64 {
    numbers.iter()
        .filter(|n| num == *n)    
        .count() as i64
}
