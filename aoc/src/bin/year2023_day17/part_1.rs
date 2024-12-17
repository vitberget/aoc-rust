use std::cmp::min;
use std::collections::{VecDeque, HashMap};
use std::fmt::Display;

use anyhow::Context;

use crate::structs::position::Position;
use crate::structs::heat_map::HeatMap;

#[derive(Clone)]
struct Path {
    path: VecDeque<(Position, u32)>,
}

impl Path {
    fn last(&self, reverse_index: usize) -> Position {
        self.path[self.path.len()-1-reverse_index].0
    }

    fn len(&self) -> usize {
        self.path.len()
    }

    fn total_heat(&self) -> u32 {
        self.path.iter()
            .map(|item| item.1)
            .sum()
    }

    fn extend(&mut self, position: (Position, u32)) {
        self.path.push_back(position)
    }
}

impl Display for Path {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("<")?;
        for (position,heat) in &self.path {
            formatter.write_fmt(format_args!("[{},{}] {}, ", position.x, position.y, heat))?;
        }
        formatter.write_str(">")
    }
}

pub async fn solve_part_1(heat_map: &HeatMap) -> anyhow::Result<u32> {
    let target_position = Position::new(heat_map.width as i32 -1, heat_map.height as i32 -1);
    let (mut paths, mut heats) = init_first_paths(heat_map)?;
    let mut iteration = 0;

    while all_paths_not_at(&paths, target_position) {
        println!();
        println!("Iteration {iteration}");
        println!("Paths:");
        paths.iter().for_each(|path| {println!("  {path}");});
        iteration += 1;

        paths = paths.into_iter()
            .flat_map(|path| extend_path(path, &target_position, heat_map))
            .collect();

        paths.iter().for_each(|path|{
            let path_heat = path.total_heat();
            let last_position = path.last(0);
            match heats.get_mut(&last_position) {
                Some(heat) => { *heat = min(*heat, path_heat); } 
                None => { heats.insert(last_position, path_heat); }
            };
        });

        if let Some(final_heat) = heats.get(&target_position) {
            paths.retain(|path| &path.total_heat() <= final_heat);
        }

        paths.retain(|path| {
            let mut current_heat: u32 = 0;
            for (position, heat) in &path.path {
                current_heat += heat;
                if heats.get(position).unwrap() < &current_heat {
                    return false;
                }
            }

            true
        });

    }

    Ok(paths.first().context("No result")?.total_heat())
}

fn all_paths_not_at(paths: &[Path], target_position: Position) -> bool {
    paths.iter().any(|path| path.last(0) != target_position)
}

fn extend_path(path: Path, target_position: &Position, heat_map: &HeatMap) -> Vec<Path> {
    let last_position = path.last(0);
    if &last_position == target_position {
        return vec![path]
    }

    let mut extensions: Vec<(Position, u32)> = vec![];

    let direction = path.last(1) - last_position;
    let can_go_straight: bool = path.len() < 3 
        || direction != path.last(2) - path.last(1);

    println!("Extending go_straigt = {can_go_straight}, direction={direction}, {path}");
    if can_go_straight {
        let new_position = last_position + direction;
        if let Some(new_heat) = heat_map.get(&new_position) {
            extensions.push((new_position, *new_heat))
        }
    }

    println!("  after can go straight {:?}", extensions);

    if direction.x == 0 {
        let new_position = last_position + Position::new(1,0);
        if let Some(new_heat) = heat_map.get(&new_position) {
            extensions.push((new_position, *new_heat))
        }
        let new_position = last_position + Position::new(-1,0);
        if let Some(new_heat) = heat_map.get(&new_position) {
            extensions.push((new_position, *new_heat))
        }
    }
    println!("  after x test {:?}", extensions);
    if direction.y == 0 {
        let new_position = last_position + Position::new(0,1);
        if let Some(new_heat) = heat_map.get(&new_position) {
            extensions.push((new_position, *new_heat))
        }
        let new_position = last_position + Position::new(0,-1);
        if let Some(new_heat) = heat_map.get(&new_position) {
            extensions.push((new_position, *new_heat))
        }
    }
    println!("  after y test {:?}", extensions);
    let ext: Vec<Path> = extensions.iter().map(|position| {
        let mut new_path = path.clone();
        new_path.extend(*position);
        new_path
    }).collect();

    ext.iter().for_each(|path| {println!("  {path}")});
    ext
}


fn init_first_paths(heat_map: &HeatMap) -> anyhow::Result<(Vec<Path>, HashMap<Position, u32>)> {
    let origin = Position::new(0,0);
    let south = Position::new(0,1);
    let east = Position::new(1,0);

    let origin_heat = heat_map.get(&origin).context("No origin position")?;
    let south_heat = heat_map.get(&south).context("No position south of origin")?;
    let east_heat = heat_map.get(&east).context("No position east of origin")?;

    let south_path = Path {
        path: VecDeque::from(
                  [(origin.to_owned(), *origin_heat),
                  (south, *south_heat) ])};

    let east_path = Path {
        path: VecDeque::from(
                  [(origin.to_owned(), *origin_heat),
                  (east, *east_heat) ])};

    let heats: HashMap<Position, u32> = HashMap::from(
        [(origin, *origin_heat),
        (south, *origin_heat + *south_heat),
        (east, *origin_heat + *east_heat), ]);

    Ok((vec![south_path, east_path], heats))
}

#[cfg(test)]
mod tests {
    use crate::parse::text_to_heat_map;

    use super::*;

    #[tokio::test]
    #[ignore = "not completed"]
    async fn test_example_year2023_day17() {
        let heat_map = text_to_heat_map(include_str!("example.txt")).unwrap();
        assert_eq!(solve_part_1(&heat_map).await.unwrap(), 102);
    }
}
