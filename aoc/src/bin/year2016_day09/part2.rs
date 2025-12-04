use aoc_procmacros::aoc_profile;
use separator::usize;

#[aoc_profile]
pub fn part2(text: &str) -> anyhow::Result<usize> {
    let r = text.trim().lines()
        .map(multi_decompress)
        .sum();

    Ok(r)
}

pub fn multi_decompress(text: &str) -> usize {
    let mut left_marker: usize = 0;
    let mut right_marker: usize = 0;

    let mut in_marker = false;
    let mut in_right_marker = false;

    let mut res: usize = 0;

    let chars: Vec<char> = text.chars().collect();
    let mut position: usize = 0;

    while let Some(ch) = chars.get(position) {
        if in_marker {
            match ch {
                ')' => {
                    in_marker = false;
                    position+=1;
                    let repeat_chars: &String = &chars[position..position + left_marker].iter().collect();
                    let multi = multi_decompress(repeat_chars);

                    for _ in 0..right_marker {
                        res += multi;
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
        } else {
            // res.push(*ch);
            res += 1;
        }
        position+=1;
    } 
    
    res
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_decompress() {
        assert_eq!(multi_decompress("(3x3)XYZ"), "XYZXYZXYZ".len());
        assert_eq!(multi_decompress("X(8x2)(3x3)ABCY"), "XABCABCABCABCABCABCY".len());
        assert_eq!(multi_decompress("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
        assert_eq!(multi_decompress("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"), 445);
    }
}
