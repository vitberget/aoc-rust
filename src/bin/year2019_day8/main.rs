use std::{env::args, fs::read_to_string};

use anyhow::bail;
use aoc_utils::get_aoc_filename;
use itertools::{izip, Itertools};

pub fn main() -> anyhow::Result<()> {
    let filename = get_aoc_filename(args(), 2019, 8);
    let text = read_to_string(filename)?;
    let text = text.trim();

    let p1 = solve_part_1(text)?;
    println!("Part 1: {p1}");

    solve_part_2(text)?;
    Ok(())
}

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn solve_part_1(text: &str) -> anyhow::Result<usize> {
    let (_, zero_layer) = text.as_bytes()
        .chunks(WIDTH * HEIGHT)
        .map(|chunk| (count_items(chunk, 0x30), chunk))
        .sorted_by(|(c1,_),(c2,_)| c1.cmp(c2))
        .next()
        .unwrap();

    Ok(count_items(zero_layer, 0x31) * count_items(zero_layer, 0x32))
}

fn count_items(chunk: &[u8], needle: u8) -> usize {
    chunk.iter()
        .filter(|hay| **hay == needle)
        .count()
}


fn solve_part_2(text: &str) -> anyhow::Result<()> {
    let layers = text.as_bytes()
        .chunks(WIDTH*HEIGHT)
        .collect_vec();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = x + y * WIDTH;

            match layers.iter()
                .map(|pixels| pixels[idx])
                .find(|pixel| *pixel != 0x32) {
                    Some(0x30) => print!(" "),
                    Some(0x31) => print!("X"),
                    n => bail!("Not a pixel {:?}", n)
                }
        }
        println!();
    }

    Ok(())
}

