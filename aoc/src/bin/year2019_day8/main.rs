use std::env::args;
use std::fs::read_to_string;

use anyhow::bail;
use aoc_utils::get_aoc_filename;
use itertools::Itertools;

pub fn main() -> anyhow::Result<()> {
    let filename = get_aoc_filename(args(), 2019, 8);
    let text = read_to_string(filename)?;
    let text = text.trim();

    let p1 = solve_part_1(text)?;
    println!("Part 1: {p1}");

    println!();
    solve_part_2(text)?;
    Ok(())
}

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const PIXEL_COUNT: usize = WIDTH * HEIGHT;

fn solve_part_1(text: &str) -> anyhow::Result<usize> {
    const CHAR_0: u8 = 0x30;
    const CHAR_1: u8 = 0x31;
    const CHAR_2: u8 = 0x32;

    let (_, zero_layer) = text.as_bytes()
        .chunks(PIXEL_COUNT)
        .map(|chunk| (count_items(chunk, CHAR_0), chunk))
        .sorted_by(|(c1,_),(c2,_)| c1.cmp(c2))
        .next()
        .unwrap();

    let ones = count_items(zero_layer, CHAR_1);
    let twos = count_items(zero_layer, CHAR_2);

    Ok(ones * twos)
}

fn count_items(chunk: &[u8], needle: u8) -> usize {
    chunk.iter()
        .filter(|hay| **hay == needle)
        .count()
}

fn solve_part_2(text: &str) -> anyhow::Result<()> {
    const BLACK: u8 = 0x30;
    const WHITE: u8 = 0x31;
    const TRANSPARENT: u8 = 0x32;

    let layers = text.as_bytes()
        .chunks(PIXEL_COUNT)
        .collect_vec();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel_index = x + y * WIDTH;

            let pixel = layers.iter()
                .map(|pixels| pixels[pixel_index])
                .find(|pixel| *pixel != TRANSPARENT);

            match pixel {
                Some(BLACK) => print!(" "),
                Some(WHITE) => print!("█"),
                n => bail!("Not a pixel {:?}", n)
            }
        }
        println!();
    }

    Ok(())
}
