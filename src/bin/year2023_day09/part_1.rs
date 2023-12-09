use itertools::Itertools;

pub fn solve_part_1(numbers: &[Vec<i128>]) -> i128 {
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
       .map(|line| line.last().unwrap())
       .sum()
}

fn all_zeroes(numbers: &[i128]) -> bool {
    for n in numbers {
        if n != &0 {
            return false;
        }
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_part1_line() {
        assert_eq!(solve_line(&vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(solve_line(&vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(solve_line(&vec![10, 13, 16, 21, 30, 45]), 68);
    }
}
