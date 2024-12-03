use anyhow::Context;

pub fn text_to_numbers(text: &str) -> anyhow::Result<(Vec<i64>, Vec<i64>)> {
    let mut left_numbers: Vec<i64> = vec![];
    let mut right_numbers: Vec<i64> = vec![];
    for line in text.lines() {
        let mut iter = line.split_whitespace();
        let left: i64 = iter.next().context("Missing")?.parse()?;
        let right: i64 = iter.next().context("Missing")?.parse()?;

        left_numbers.push(left);
        right_numbers.push(right);
    }

    Ok((left_numbers, right_numbers))
}
