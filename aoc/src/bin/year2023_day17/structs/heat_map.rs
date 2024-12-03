use std::fmt::Display;
use std::collections::HashMap;

use super::position::Position;

pub(crate) struct HeatMap {
    pub height: usize,
    pub width: usize,

    pub map: HashMap<Position, u32>
}

impl HeatMap {
    pub fn get(&self, position: &Position) -> Option<&u32> {
        self.map.get(position)
    }
}

impl Display for HeatMap {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_fmt(format_args!("Height: {} Width: {}\n", self.height, self.width))?;  
        for y in 0..self.height {
            for x in 0..self.width {
                if x==0 && y!=0 { formatter.write_str("\n")?; }

                if let Some(digit) = self.get(&Position::new(x as i32,y as i32)) {
                    formatter.write_fmt(format_args!("{}", digit))?;
                } else {
                    formatter.write_str("?")?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::text_to_heat_map;

    #[test]
    #[ignore= "Visual test of output"]
    fn test_display_heatmap() {
        let map = text_to_heat_map(include_str!("../example.txt")).unwrap();
        println!("{map}");
    }

}
