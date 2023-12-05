use anyhow::bail;

use crate::structs::Almenac;

pub fn solve_part_1(almenac: &Almenac) -> anyhow::Result<i64> {
    let result = almenac.seeds.iter()
        .map(|seed| translate_entire_seed(*seed, almenac))
        .min();

    match result {
        Some(number) => Ok(number),
        None => bail!("No solution?")
    }
}

fn translate_entire_seed(seed: i64, almenac: &Almenac) -> i64 {
    let result = almenac.seed_to_soil.translate(seed);
    let result = almenac.soil_to_fertilizer.translate(result);
    let result = almenac.fertilizer_to_water.translate(result);
    let result = almenac.water_to_light.translate(result);
    let result = almenac.light_to_temperature.translate(result);
    let result = almenac.temperature_to_humidity.translate(result);
    almenac.humidity_to_location.translate(result)
}
