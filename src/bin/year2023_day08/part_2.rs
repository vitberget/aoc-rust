use anyhow::Context;
use tokio::task::JoinSet;

use crate::structs::{Location, Network};

pub async fn solve_part_2(instructions: &str, network: &Network) -> anyhow::Result<i128> {
    let mut join_set: JoinSet<i128> = JoinSet::new();
    for location in network.keys().filter(|location| location.ends_with('A')) {
        join_set.spawn(solve_one_path(location.to_owned(), instructions.to_owned(), network.clone()));
    }

    let mut factors: Vec<i128> = vec![];

    while let Some(result) = join_set.join_next().await {
        if let Ok(to_z) = result {
            factors.push(to_z);
        }
    }

    factors.into_iter().reduce(least_common_denominator).context("Failed to reduce")
}

fn greatest_common_denominator(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn least_common_denominator(a: i128, b: i128) -> i128 {
    a * b / greatest_common_denominator(a, b)
}

async fn solve_one_path(start_location: Location, instructions: String, network: Network) -> i128 {
    let mut char_iter = instructions.chars(); 
    let mut step: i128 = 0;
    let mut location = start_location.to_owned();

    let mut to_z: i128 = 0;

    while to_z == 0 {
        step += 1;        
        let direction = match char_iter.next() {
            Some(letter) => letter,
            None => {
                char_iter = instructions.chars();
                char_iter.next().unwrap()
            }
        };
        let next_step = network.get(&location).unwrap();
        location = match direction {
            'L' => next_step.0,
            'R' => next_step.1,
            n => panic!("Unexpected direction {n}")
        };

        if to_z == 0 && location.ends_with('Z') { 
            to_z = step;
        }
    }

    to_z
}
