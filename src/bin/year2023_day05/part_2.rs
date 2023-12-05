use std::sync::Arc;

use tokio::task::JoinSet;

use crate::structs::Almenac;

pub async fn solve_part_2(almenac: &Almenac) -> anyhow::Result<i64> {
    let mut seed_iter = almenac.seeds.iter();

    let mut extended_seed: Vec<(i64,i64)> = vec![];

    while let Some(start) = seed_iter.next() {
        if let Some(size) = seed_iter.next() {
            extended_seed.push((*start, start+size));
        } else {break}
    } 

    extended_seed.sort();

    let a_clone = almenac.clone().to_owned();
    let arced_almenac = Arc::new(a_clone);
    let mut join_set = JoinSet::new();

    for (start,end) in extended_seed {
        let aa = Arc::clone(&arced_almenac);
        join_set.spawn(calculate(start, end, aa));
    }


    let mut min: i64 = i64::MAX;
    while let Some(res) = join_set.join_next().await {
        let location = res?;
        if location < min {
            min = location;
        }
    }
    Ok(min)
}

async fn calculate(start: i64, end: i64, almenac: Arc<Almenac>) -> i64 {
    let mut seed = start;
    let mut min = i64::MAX;
    while seed < end {
        let location = translate_entire_seed(seed, &almenac).await;
        if location < min {
            min = location;
        }
        seed += 1;
    }
    min
}


async fn translate_entire_seed(seed: i64, almenac: &Almenac) -> i64 {
    let result = almenac.seed_to_soil.translate(seed);
    let result = almenac.soil_to_fertilizer.translate(result);
    let result = almenac.fertilizer_to_water.translate(result);
    let result = almenac.water_to_light.translate(result);
    let result = almenac.light_to_temperature.translate(result);
    let result = almenac.temperature_to_humidity.translate(result);
    almenac.humidity_to_location.translate(result)
}


#[cfg(test)]
mod tests {
    use crate::parse::parse;

    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn example_part_2() {
        let text = include_str!("example.txt");
        let almenac = parse(&text).unwrap();
        let result = solve_part_2(&almenac).await.unwrap();
        assert_eq!(result, 46);
    }
}
