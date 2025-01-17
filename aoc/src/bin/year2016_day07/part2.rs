use std::collections::HashSet;

use aoc_procmacros::aoc_profile;
use itertools::Itertools;

use crate::part1::{Ipv7Address, Ipv7Part};

#[aoc_profile]
pub fn part2(text: &str) -> anyhow::Result<usize> {
    let r = text.trim()
        .lines()
        .map(Ipv7Address::parse)
        .filter(|ipv7| ipv7.is_valid_ssl())
        .count();

    Ok(r)
}

impl Ipv7Address {
    pub fn is_valid_ssl(&self) -> bool {
        let normals: HashSet<[char; 3]> = self.parts.iter()
            .filter(|(ip_type, _)| *ip_type == Ipv7Part::Normal)
            .flat_map(|(_, chars)| get_threes(chars, Inverted::No))
            .collect();
        let hypernets: HashSet<[char; 3]> = self.parts.iter()
            .filter(|(ip_type, _)| *ip_type == Ipv7Part::Hypernet)
            .flat_map(|(_, chars)| get_threes(chars, Inverted::Yes))
            .collect();

        normals.intersection(&hypernets).count() > 0
    }
}

pub enum Inverted { Yes, No }

pub fn get_threes(chars: &[char], inverted: Inverted) -> Vec<[char; 3]> {
    chars.iter().tuple_windows()
        .filter(|(c0, c1, c2)| c0 == c2 && c0 != c1)
        .map(|(c0, c1, _)| match inverted {
            Inverted::Yes => [*c1, *c0, *c1],
            Inverted::No => [*c0, *c1, *c0]
        }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
       assert!(Ipv7Address::parse("aba[bab]xyz").is_valid_ssl()); 
       assert!(! Ipv7Address::parse("xyx[xyx]xyx").is_valid_ssl()); 
       assert!(Ipv7Address::parse("aaa[kek]eke").is_valid_ssl()); 
       assert!(Ipv7Address::parse("zazbz[bzb]cdb").is_valid_ssl()); 
    }
}
