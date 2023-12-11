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

    let arced_almenac = Arc::new(almenac.clone().to_owned());
    let mut join_set = JoinSet::new();

    for (start,end) in extended_seed {
        join_set.spawn(calculate(start, end, Arc::clone(&arced_almenac)));
    }

    let mut min: i64 = i64::MAX;
    while let Some(res) = join_set.join_next().await {
        min = min.min(res?);
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
    let soil = almenac.seed_to_soil.translate(seed);
    let fertilizer = almenac.soil_to_fertilizer.translate(soil);
    let water = almenac.fertilizer_to_water.translate(fertilizer);
    let light = almenac.water_to_light.translate(water);
    let temperature = almenac.light_to_temperature.translate(light);
    let humidity = almenac.temperature_to_humidity.translate(temperature);
    almenac.humidity_to_location.translate(humidity)
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
