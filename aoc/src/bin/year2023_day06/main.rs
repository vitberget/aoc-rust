use std::iter::zip;

const PUZZLE: &str = 
"Time:        42     89     91     89
Distance:   308   1170   1291   1467" ;

pub fn main() -> anyhow::Result<()> {
    let p1 = solve_part_1(PUZZLE)?;
    println!("Part 1 {p1}");

    let p2 = solve_part_2(PUZZLE)?;
    println!("Part 2 {p2}");

    Ok(())
}

pub fn solve_part_1(text: &str) -> anyhow::Result<i64> {
    let numbers: Vec<Vec<i64>> = text.lines()
        .map(line_to_numbers)
        .collect();

    Ok(zip(numbers.first().unwrap(), numbers.get(1).unwrap())
       .map(|(time, distancce)| count_beating_distance(time, distancce))
       .reduce(|a,n| a*n).unwrap())
}


/// Works fast, otherwise perhaps find one edge of where it starts to
/// beat the distance with binary search logic, and then mirror to the
/// other side (since it is symmetrical)
fn count_beating_distance(total_time: &i64, distance_to_beat: &i64) -> i64 {
    (1..=*total_time)
        .map(|time| time * (total_time - time))
        .filter(|distance| distance > distance_to_beat)
        .count()
        .try_into().unwrap()
}

fn line_to_numbers(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .skip(1)
        .map(|word| word.parse().unwrap())
        .collect()
}

pub fn solve_part_2(text: &str) -> anyhow::Result<i64> {
    let numbers: Vec<i64> = text.lines()
        .map(concat_line_to_number)
        .collect();

    let time = numbers.first().unwrap();
    let distance = numbers.get(1).unwrap();

    Ok(count_beating_distance(time, distance))
}

fn concat_line_to_number(line: &str) -> i64 {
    line.split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>().join("")
        .parse().unwrap()
}
