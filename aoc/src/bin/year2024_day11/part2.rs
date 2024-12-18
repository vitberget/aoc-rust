use std::collections::HashMap;

use aoc_procmacros::log_duration;

#[log_duration]
pub fn blink_many_times(text: &str, times: usize) -> usize {
    let mut arragment = text_to_arragment(text);

    for _ in 0..times {
        arragment = blink(&arragment);
    }

    arragment.values().sum()
}

#[log_duration(counter)]
fn blink(arragment: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut result: HashMap<u64, usize> = HashMap::new();

    for (number, size) in arragment {
        if *number == 0 {
            add_number(&mut result, 1, size);
        } else {
            let digits_in_number = number.ilog10() + 1;

            if digits_in_number % 2 == 0 {
                let divider = 10_u64.pow(digits_in_number / 2);
                add_number(&mut result, number / divider, size);
                add_number(&mut result, number % divider, size);
            } else  {
                add_number(&mut result, number * 2024, size);
            }
        }
    }

    result
}

fn text_to_arragment(text: &str) -> HashMap<u64, usize> {
    let numbers: Vec<u64> = text.trim()
        .split(" ")
        .flat_map(|word| word.parse::<u64>())
        .collect();
    
    vec_to_hashmap(numbers)
}

fn vec_to_hashmap(numbers: Vec<u64>) -> HashMap<u64, usize> {
    let mut arragment: HashMap<u64, usize> = HashMap::new();

    for number in numbers {
        match arragment.get_mut(&number) {
            Some(count) => { *count += 1; },
            None => { arragment.insert(number, 1); }
        }
    }

    arragment
}

fn add_number(arragment: &mut HashMap<u64, usize>, number: u64, count: &usize) {
    arragment.entry(number)
        .and_modify(|s| *s += count)
        .or_insert(*count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blink_many() {
        assert_eq!(blink_many_times("125 17", 6), 22);
        assert_eq!(blink_many_times("125 17", 25), 55312);
    }

    #[test]
    fn blinks() {
        let blink_0 = vec_to_hashmap(vec![125, 17]);
        let blink_1 = vec_to_hashmap(vec![253000, 1, 7]);
        let blink_2 = vec_to_hashmap(vec![253, 0, 2024, 14168]);
        let blink_3 = vec_to_hashmap(vec![512072, 1, 20, 24, 28676032]);
        let blink_4 = vec_to_hashmap(vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]);
        let blink_5 = vec_to_hashmap(vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]);
        let blink_6 = vec_to_hashmap(vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2]);

        assert_eq!(blink(&blink_0), blink_1);
        assert_eq!(blink(&blink_1), blink_2);
        assert_eq!(blink(&blink_2), blink_3);
        assert_eq!(blink(&blink_3), blink_4);
        assert_eq!(blink(&blink_4), blink_5);
        assert_eq!(blink(&blink_5), blink_6);
    }
}
