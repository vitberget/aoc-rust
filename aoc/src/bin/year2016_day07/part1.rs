
use aoc_procmacros::aoc_profile;
use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub enum Ipv7Part {
    Normal,
    Hypernet,
}

#[derive(Debug, PartialEq)]
pub struct Ipv7Address {
    pub parts: Vec<(Ipv7Part, Vec<char>)>
}

impl Ipv7Address {
    pub fn parse(line: &str) -> Self {
        let mut parts: Vec<(Ipv7Part, Vec<char>)> = vec![];
        let mut ip_type = Ipv7Part::Normal;
        let mut current_part: Vec<char> = vec![];

        for ch in line.chars() {
            match ch {
                '[' => {
                    if ! current_part.is_empty() {
                        parts.push((ip_type, current_part));
                        current_part = vec![];
                    }
                    ip_type = Ipv7Part::Hypernet
                },
                ']' => {
                    if ! current_part.is_empty() {
                        parts.push((ip_type, current_part));
                        current_part = vec![];
                    }
                    ip_type = Ipv7Part::Normal
                },

                other => current_part.push(other)
            }
        }
        if ! current_part.is_empty() {
            parts.push((ip_type, current_part));
        }

        Self { parts }
    } 

    pub fn is_valid_tls(&self) -> bool {
        let normals = self.parts.iter()
            .filter(|(ip_type, _)| *ip_type == Ipv7Part::Normal)
            .any(|(_, chars)| is_abba(chars));

        normals && self.parts.iter()
            .filter(|(ip_type, _)| *ip_type == Ipv7Part::Hypernet)
            .all(|(_, chars)| ! is_abba(chars))
    }
}


#[aoc_profile]
pub fn part1(text: &str) -> anyhow::Result<usize> {
    let r = text.trim()
        .lines()
        .map(Ipv7Address::parse)
        .filter(|ipv7| ipv7.is_valid_tls())
        .count();

    Ok(r)
}

fn is_abba(text: &[char]) -> bool {
    text.iter()
        .tuple_windows()
        .any(|(c0, c1, c2, c3)| c0 == c3 && c1 == c2 && c0 != c1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Ipv7Address::parse("abba[mnop]qrst"), 
            Ipv7Address { 
                parts: vec![
                    (Ipv7Part::Normal, vec!['a', 'b', 'b', 'a']),
                    (Ipv7Part::Hypernet, vec!['m', 'n', 'o', 'p']),
                    (Ipv7Part::Normal, vec!['q', 'r', 's', 't']),
                ]
            });
    }

    #[test]
    fn test_abba() {
        assert!(is_abba(&"abba".chars().collect_vec()));
        assert!(is_abba(&"bvabba".chars().collect_vec()));
        assert!(is_abba(&"bvabbasfd".chars().collect_vec()));
        assert!(is_abba(&"abbasfd".chars().collect_vec()));
        assert!(!is_abba(&"abcdefg".chars().collect_vec()));
        assert!(!is_abba(&"aaaa".chars().collect_vec()));
    }

    #[test]
    fn is_valid_tls() -> anyhow::Result<()> {
        assert!(Ipv7Address::parse("abba[mnop]qrst").is_valid_tls());
        assert!(! Ipv7Address::parse("abcd[bddb]xyyx").is_valid_tls());
        assert!(! Ipv7Address::parse("aaaa[qwer]tyui").is_valid_tls());
        assert!(Ipv7Address::parse("ioxxoj[asdfgh]zxcvbn").is_valid_tls());
        Ok(())
    }
}
