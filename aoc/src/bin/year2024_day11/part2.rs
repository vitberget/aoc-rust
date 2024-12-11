use std::collections::HashMap;

pub fn blink_many_times(text: &str, times: usize) -> usize {
    let numbers: Vec<u64> = text.trim()
        .split(" ")
        .flat_map(|word| word.parse::<u64>())
        .collect();

    let mut arragment: HashMap<u64, usize> = HashMap::new();
    for n in numbers {
        arragment.entry(n).and_modify(|s| *s +=1).or_insert(1);
    } 

    for _ in 0..times {
        arragment = blink(&arragment);
    }

    arragment.values().sum()
}

fn blink(arragment: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut result: HashMap<u64, usize> = HashMap::new();

    for (number, size) in arragment {
        if *number == 0 {
            result.entry(1).and_modify(|s| *s += size).or_insert(*size);
        } else {
            let digits_in_number = number.ilog10() + 1;
            if (digits_in_number) % 2 == 0 {
                let divider = 10_u64.pow(digits_in_number/2);
                result.entry(number / divider).and_modify(|s| *s += size).or_insert(*size);
                result.entry(number % divider).and_modify(|s| *s += size).or_insert(*size);
            } else  {
                result.entry(number * 2024).and_modify(|s| *s += size).or_insert(*size);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blink_many() {
        assert_eq!(blink_many_times("125 17", 6), 22);
        assert_eq!(blink_many_times("125 17", 25), 55312);
    }
}
