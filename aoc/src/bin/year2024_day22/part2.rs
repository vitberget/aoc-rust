use std::collections::{HashMap, HashSet};

use aoc_procmacros::aoc_profile;
use itertools::Itertools;

use crate::part1::evolve_secret_number;

type Diffs = (i8, i8, i8, i8);

#[aoc_profile]
pub fn solve_part_2(text: &str) -> anyhow::Result<i64> {
    let mut hashy: HashMap<Diffs, i64> = HashMap::new();
    text.lines()
        .map(|line| line.parse::<u64>().unwrap())
        .for_each(|number| do_hashy(&mut hashy, number));
        
    Ok(*hashy.values().max().unwrap())
}

fn do_hashy(hashy: &mut HashMap<Diffs, i64>, number: u64) {
    let mut secret_number: u64 = number.to_owned();
    let mut visited: HashSet<Diffs> = HashSet::new();
    let mut price:i8 = (number%10) as i8;
    let mut diffs: Vec<i8> = vec![];

    for _ in 0..2000 {
        secret_number = evolve_secret_number(&secret_number);
        let old_price = price;
        price = (secret_number % 10) as i8;
        diffs.push(price - old_price);

        if diffs.len() > 4 {
            diffs.remove(0);
        }

        if diffs.len() == 4 {
            let key = (diffs[0], diffs[1], diffs[2], diffs[3]);
            if ! visited.contains(&key) {
                hashy.entry(key)
                    .and_modify(|s| *s += price as i64)
                    .or_insert(price.into());

                visited.insert(key);
            }
        } 
    }
}


#[aoc_profile]
pub fn solve_part_2_old(text: &str) -> anyhow::Result<i64> {
    let series: Vec<HashMap<Diffs, i64>> = text.lines()
        .map(|line| line.parse::<u64>().unwrap())
        .map(|number| get_hashy(&number, 2000))
        .collect();

    let possible_seqs: HashSet<&Diffs> = all_keys(&series);

    Ok(possible_seqs.iter()
        .map(|possible| get_sum_of(possible, &series))
        .max()
        .unwrap())
}

#[aoc_profile]
pub fn get_hashy(number: &u64, amount: usize) -> HashMap<Diffs, i64> {
    let mut result: HashMap<Diffs, i64> = HashMap::new();

    let mut secret_number: u64 = number.to_owned();
    let mut price:i8 = (number%10) as i8;
    let mut diffs: Vec<i8> = vec![];

    for _ in 0..amount {
        secret_number = evolve_secret_number(&secret_number);
        let old_price = price;
        price = (secret_number % 10) as i8;
        diffs.push(price - old_price);

        if diffs.len() > 4 {
            diffs.remove(0);
        }

        if diffs.len() == 4 {
            let key = (diffs[0], diffs[1], diffs[2], diffs[3]);
            result.entry(key).or_insert(price.into());
        } 
    }

    result
}

#[aoc_profile]
fn all_keys(maps: &[HashMap<Diffs, i64>]) -> HashSet<&Diffs> {
    maps.iter()
        .map(|s| s.keys())
        .flat_map(|k| k.collect_vec())
        .collect()
}

#[aoc_profile]
fn get_sum_of(possible: &Diffs, series: &[HashMap<Diffs, i64>]) -> i64 {
    series.iter()
        .flat_map(|s| s.get(possible))
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashy() {
        let hashy = get_hashy(&1, 1999);
        println!("hashy.len() {}", hashy.len());
        assert!(hashy.len() > 100);
    }
    #[test]
    fn test_hashy_123() {
        let hashy = get_hashy(&123, 9);
        println!("hashy {:?}", hashy);
        println!("hashy len {:?}", hashy.len());
        assert_eq!(hashy.len(), 6);
    }
}
