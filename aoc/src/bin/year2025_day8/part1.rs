use std::collections::HashSet;

use aoc_procmacros::aoc_profile;
use itertools::Itertools;

use crate::structs::{calculate_distances, find_circuit, JunctionBox};

#[aoc_profile]
pub fn part1(text: &str, connections: usize) -> anyhow::Result<usize> {
    let mut circuits = JunctionBox::parse_text(text);
    let junction_distances = calculate_distances(&circuits);

    for (junction_box_a, junction_box_b) in junction_distances.iter().take(connections) {
        let circuit_a = find_circuit(&circuits, junction_box_a);
        let circuit_b = find_circuit(&circuits, junction_box_b);

        if circuit_a != circuit_b {
            circuits.retain(|circuit| circuit!=&circuit_a && circuit!=&circuit_b);
            let new_circuit: HashSet<JunctionBox> = circuit_a.union(&circuit_b).map(|x| x.to_owned()).collect();

            circuits.push(new_circuit.to_owned());
        }
    }

    let size: usize = circuits.iter()
        .map(|c| c.len())
        .sorted_by(|a,b| b.cmp(a))
        .take(3)
        .reduce(|a,b| a*b)
        .unwrap();

    Ok(size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        const EXAMPLE: &str = include_str!("example.txt");
        let result = part1(EXAMPLE, 10)?;
        assert_eq!(result, 40);
        Ok(())
    }
}
