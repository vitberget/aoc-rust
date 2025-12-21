use std::collections::HashSet;

use anyhow::bail;
use aoc_procmacros::aoc_profile;
use itertools::Itertools;

use crate::part1::Coordinate;

#[derive(Debug)]
pub struct Horizontal {
    pub min_x: usize,
    pub max_x: usize,
    pub y: usize
}

#[derive(Debug)]
pub struct Vertical {
    pub min_y: usize,
    pub max_y: usize,
    pub x: usize
}

    #[derive(Debug)]
    struct Data {
        min_x: usize,
        max_x: usize,
        min_y: usize,
        max_y: usize,
        size: usize
    }
#[aoc_profile]
pub fn part2(text: &str) -> anyhow::Result<usize> {
    let coordinates: Vec<Coordinate> = Coordinate::parse_text(text);

    let mut extended = coordinates.clone();
    extended.push(coordinates[0].clone());
    let horizontal = get_horizontal(&extended);
    let vertical = get_vertical(&extended);

    let mut rects: Vec<Data> = vec![];

    for i in 0..coordinates.len() {
        let coordinate_1 = &coordinates[i];
        for coordinate_2 in coordinates.iter().skip(i+1) {
            let max_x = usize::max(coordinate_1.x, coordinate_2.x);
            let min_x = usize::min(coordinate_1.x, coordinate_2.x);
            let dx = max_x - min_x + 1;

            let max_y = usize::max(coordinate_1.y, coordinate_2.y);
            let min_y = usize::min(coordinate_1.y, coordinate_2.y);
            let dy = max_y - min_y + 1;
            let dsize = dx * dy;

            let data = Data { min_x, max_x, min_y, max_y, size: dsize };
            rects.push(data);
        }
    }

    rects.sort_by(|a,b| b.size.cmp(&a.size));
    // rects.push(Data { 
    //     min_x: 2, 
    //     max_x: 9, 
    //     min_y: 3, 
    //     max_y: 5, 
    //     size: 24 
    // });
    for rect in rects {
        todo!();
        // println!("rect {rect:?}");
        // if is_below(&rect, &horizontal, &vertical) &&
        //    is_above(&rect, &horizontal, &vertical) &&
        //    is_left(&rect, &horizontal, &vertical) &&
        //    is_right(&rect, &horizontal, &vertical) {
        //        return Ok(rect.size);
        // }
    }


    bail!("should not happen");
}

fn get_horizontal(coordinates: &[Coordinate]) -> Vec<Horizontal> {
    coordinates.windows(2)
        .filter(|arr| arr[0].y == arr[1].y)
        .map(|arr| Horizontal { min_x: usize::min(arr[0].x, arr[1].x), max_x: usize::max(arr[0].x, arr[1].x), y: arr[0].y } )
        .collect()
}
fn get_vertical(coordinates: &[Coordinate]) -> Vec<Vertical> {
    coordinates.windows(2)
        .filter(|arr| arr[0].x == arr[1].x)
        .map(|arr| Vertical { min_y: usize::min(arr[0].y, arr[1].y), max_y: usize::max(arr[0].y, arr[1].y), x: arr[0].x } )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        const EXAMPLE: &str = include_str!("example.txt");
        let result = part2(EXAMPLE)?;
        assert_eq!(result, 24);
        Ok(())
    }
}
