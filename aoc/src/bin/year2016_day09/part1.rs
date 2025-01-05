use aoc_procmacros::aoc_profile;
use separator::usize;

#[aoc_profile]
pub fn part1(text: &str) -> anyhow::Result<usize> {
    let r = text.trim().lines()
        .map(decompress)
        .map(|dt| dt.len())
        .sum();

    Ok(r)
}

pub fn decompress(text: &str) -> String {
    let mut left_marker: usize = 0;
    let mut right_marker: usize = 0;

    let mut in_marker = false;
    let mut in_right_marker = false;

    let mut res: Vec<char> = vec![];

    let chars: Vec<char> = text.chars().collect();
    let mut position: usize = 0;

    while let Some(ch) = chars.get(position) {
        if in_marker {
            match ch {
                ')' => {
                    in_marker = false;
                    position+=1;
                    let repeat_chars = &chars[position..position + left_marker];

                    for _ in 0..right_marker {
                        repeat_chars.iter().for_each(|ch| res.push(*ch));
                    }
                    position += left_marker - 1;
                }
                n if n.is_ascii_digit() => {
                    if in_right_marker {
                        right_marker *= 10;
                        right_marker += n.to_digit(10).unwrap() as usize;
                    } else {
                        left_marker *= 10;
                        left_marker += n.to_digit(10).unwrap() as usize;
                    }
                }
                'x' => in_right_marker = true,
                _ => {}
            }
        } else if *ch == '(' {
            in_marker = true;
            in_right_marker = false;
            left_marker = 0;
            right_marker = 0;
            in_right_marker = false;
        } else {
            res.push(*ch);
        }
        position+=1;
    } 

    res.iter().collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress() {
        assert_eq!(decompress("ADVENT"), "ADVENT");
        assert_eq!(decompress("A(1x5)BC"), "ABBBBBC");
        assert_eq!(decompress("(3x3)XYZ"), "XYZXYZXYZ");
        assert_eq!(decompress("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
        assert_eq!(decompress("(6x1)(1x3)A"), "(1x3)A");
        assert_eq!(decompress("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
    }
}
