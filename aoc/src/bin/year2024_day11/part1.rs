use aoc_procmacros::log_duration;

#[log_duration]
pub fn blink_many_times(text: &str, times: usize) -> usize {
    let mut arragment = text_to_arragment(text);

    for _ in 0..times {
        arragment = blink(&arragment);
    }

    arragment.len()
}

#[log_duration]
fn blink(arragment: &[u64]) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];

    for number in arragment {
        if *number == 0 {
            result.push(1);
        } else {
            let digits_in_number = number.ilog10() + 1;
            if digits_in_number % 2 == 0 {
                let divider = 10_u64.pow(digits_in_number / 2);
                result.push(number / divider);
                result.push(number % divider);
            } else  {
                result.push(number * 2024);
            }
        }
    }

    result
}

fn text_to_arragment(text: &str) -> Vec<u64> {
    text.trim()
        .split(" ")
        .flat_map(|word| word.parse::<u64>())
        .collect()
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
        let blink_0 = vec![125, 17];
        let blink_1 = vec![253000, 1, 7];
        let blink_2 = vec![253, 0, 2024, 14168];
        let blink_3 = vec![512072, 1, 20, 24, 28676032];
        let blink_4 = vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032];
        let blink_5 = vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32];
        let blink_6 = vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2];

        assert_eq!(blink(&blink_0), blink_1);
        assert_eq!(blink(&blink_1), blink_2);
        assert_eq!(blink(&blink_2), blink_3);
        assert_eq!(blink(&blink_3), blink_4);
        assert_eq!(blink(&blink_4), blink_5);
        assert_eq!(blink(&blink_5), blink_6);
    }
}
