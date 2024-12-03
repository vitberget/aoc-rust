use anyhow::{Context, bail};

use crate::structs::{Network, Location};

pub fn solve_part_1(instructions: &str, network: &Network) -> anyhow::Result<u128> {
    let destination = Location::new("ZZZ")?;
    let mut char_iter = instructions.chars();
    let mut location = Location::new("AAA")?;
    let mut step: u128 = 0;

    while location!=destination {
        step += 1;
        let direction = match char_iter.next() {
            Some(letter) => letter,
            None => {
                char_iter = instructions.chars();
                char_iter.next().context("Should not happen!?!")?
            }
        };

        let next_step = network.get(&location).context("Missing location")?;
        location = match direction {
            'L' => next_step.0,
            'R' => next_step.1,
            n => bail!("Unexpected direction {n}")
        }
    }

    Ok(step)
}
