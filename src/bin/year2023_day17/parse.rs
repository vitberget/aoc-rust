use std::collections::HashMap;
use std::cmp::max;

use anyhow::Context;

use crate::structs::heat_map::HeatMap;
use crate::structs::position::Position;

pub fn text_to_heat_map(text: &str) -> anyhow::Result<HeatMap> {
    let mut height: usize = 0;
    let mut width: usize = 0;
    let mut the_map: HashMap<Position, u32> = HashMap::new();

    for (y, line) in text.lines().enumerate() {
       height = max(height, y);
       for (x, ch) in line.chars().enumerate() {
           if y == 0 { width = max(width, x); }
            
           the_map.insert(Position::new(x as i32, y as i32), ch.to_digit(10).context("Not a digit")?);
       }
    };

    Ok(HeatMap { 
        height: height + 1,
        width: width +1, 
        map: the_map 
    })
}
