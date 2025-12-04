use std::str::Lines;

use aoc_procmacros::aoc_profile;
use itertools::Itertools;
use itertools::Chunk;

#[aoc_profile]
pub fn part2(text: &str) -> anyhow::Result<usize> {
    Ok(
        text.trim()
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| count_triangles(chunk))
        .sum())
}

fn count_triangles(chunk: Chunk<Lines>) -> usize {
    let mut iter = chunk
        .map(|line| line.split_whitespace()
            .flat_map(|word| word.parse::<usize>()));

    let mut triangles: usize = 0;

    if let Some(mut nums1) = iter.next()
    && let Some(mut nums2) = iter.next()
    && let Some(mut nums3) = iter.next() {
        for _ in 0..3 {
            if let Some(n1) = nums1.next()
            && let Some(n2) = nums2.next()
            && let Some(n3) = nums3.next() 
            && n1 + n2 > n3  
            && n1 + n3 > n2 
            && n2 + n3 > n1 {
                triangles += 1;
            }
        }
    }

    triangles 
}
