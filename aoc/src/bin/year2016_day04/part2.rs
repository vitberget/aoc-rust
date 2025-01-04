use std::char;

use aoc_procmacros::aoc_profile;

use crate::part1::get_sector;

#[aoc_profile]
pub fn part2(text: &str) -> anyhow::Result<usize> {

    text.trim()
        .lines()
        .map(decrypt)
        .for_each(|(name, sector)| println!("#{sector}: {name}"));

    // #548: northpole object storage oetrc
    Ok(0)
}

pub fn decrypt(line: &str) -> (String, u32) {
    let sector: u32 = get_sector(line) as u32;
    let result: String = line.chars()
        .filter(|ch| ch.is_ascii_lowercase() || *ch == '-')
        .map(|ch| match ch {
            '-' => ' ',
            ch if ch.is_ascii_lowercase() => ceasar(&ch, sector),
            _ => '!'
        }).collect();

    (result.trim().to_string(), sector)
}

pub fn ceasar(ch: &char, shift: u32) -> char {
    const A: u32 = 'a' as u32;
    const Z: u32 = 'z' as u32;
    const AZ: u32 = Z - A + 1;

    let c = (*ch as u32 - A + shift) % AZ;
    char::from_u32(c + A).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt() {
        assert_eq!(decrypt("qzmt-zixmtkozy-ivhz-343"), ("very encrypted name".to_string(), 343u32));
    }

    #[test]
    fn test_ceasar() {
        assert_eq!(ceasar(&'a',1), 'b');
        assert_eq!(ceasar(&'z',1), 'a');
    }
}
