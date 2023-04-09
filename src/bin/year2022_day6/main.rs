use std::fs::read_to_string;
use std::env::args;
use aoc_utils::get_aoc_filename;

fn main() {
    let filename = get_aoc_filename(args(), 2022, 6);
    let text = read_to_string(filename).unwrap();

    println!("part1 {}", day6(&text, 4));
    println!("part2 {}", day6(&text, 14));
}

fn day6(text: &str, different_letters: usize) -> usize {
    let letters: Vec<char> = text.chars().collect();
    let mut answer: usize = 0;

    while answer + different_letters < letters.len() && !is_this_it(&letters[answer..(answer + different_letters) as usize]) {
        answer += 1;
    }

    return answer + different_letters;
}

fn is_this_it(list: &[char]) -> bool {
    for idx in 0..list.len() {
        let c = &list[idx];

        for tc in &list[idx + 1..list.len()] {
            if c == tc { return false; }
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(day6("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(day6("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(day6("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(day6("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(day6("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn part2() {
        assert_eq!(day6("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(day6("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(day6("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(day6("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(day6("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}