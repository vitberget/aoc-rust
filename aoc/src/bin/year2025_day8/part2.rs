use std::collections::HashSet;

use aoc_procmacros::aoc_profile;

use crate::structs::{calculate_distances, find_circuit, JunctionBox};

#[aoc_profile]
pub fn part2(text: &str) -> anyhow::Result<usize> {
 let mut circuits = JunctionBox::parse_text(text);
    let junction_distances = calculate_distances(&circuits);

    for (junction_box_a, junction_box_b) in junction_distances {
        let circuit_a = find_circuit(&circuits, &junction_box_a);
        let circuit_b = find_circuit(&circuits, &junction_box_b);

        if circuit_a != circuit_b {
            circuits.retain(|circuit| circuit!=&circuit_a && circuit!=&circuit_b);

            if circuits.is_empty() {
                return Ok(junction_box_a.x * junction_box_b.x);
            }

            let new_circuit: HashSet<JunctionBox> = circuit_a.union(&circuit_b).map(|x| x.to_owned()).collect();
            circuits.push(new_circuit.to_owned());
        }
    }

    Err(anyhow::Error::msg("Should not exit loop"))
    // Err("Should not exit loop")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        const EXAMPLE: &str = include_str!("example.txt");
        let result = part2(EXAMPLE)?;
        assert_eq!(result, 25272);
        Ok(())
    }
}
