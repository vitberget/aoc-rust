use itertools::Itertools;

pub fn solve_part_2(numbers: &[Vec<i128>]) -> i128 {
    numbers.iter()
        .map(solve_line)
        .sum()
}

fn solve_line(one_line: &Vec<i128>) -> i128 {
   let mut numbers = vec![one_line.to_owned()];

   while !all_zeroes(numbers.last().unwrap()) {
        let new_line: Vec<i128> = numbers.last().unwrap().iter()
            .tuple_windows()
            .map(|(a,b)| b - a)
            .collect();
        numbers.push(new_line);
   }

   numbers.iter()
       .map(|line| line.first().unwrap())
       .map(|n| n.to_owned())
       .rev()
       .reduce(|a,n| n - a)
       .unwrap()
}

fn all_zeroes(numbers: &[i128]) -> bool {
    numbers.iter()
        .find(|n| n != &&0)
        .is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_part2_line() {
        assert_eq!(solve_line(&vec![0, 3, 6, 9, 12, 15]), -3);
        assert_eq!(solve_line(&vec![1, 3, 6, 10, 15, 21]), 0);
        assert_eq!(solve_line(&vec![10, 13, 16, 21, 30, 45]), 5);
    }
}
