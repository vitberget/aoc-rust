use crate::structs::Almenac;

pub fn solve_part_1(almenac: &Almenac) -> anyhow::Result<i64> {
    Ok(almenac.seeds.iter()
       .map(|seed| translate_entire_seed(*seed, almenac))
       .min()
       .expect("No solution"))
}

fn translate_entire_seed(seed: i64, almenac: &Almenac) -> i64 {
    let soil = almenac.seed_to_soil.translate(seed);
    let fertilizer = almenac.soil_to_fertilizer.translate(soil);
    let water = almenac.fertilizer_to_water.translate(fertilizer);
    let light = almenac.water_to_light.translate(water);
    let temperature = almenac.light_to_temperature.translate(light);
    let humidity = almenac.temperature_to_humidity.translate(temperature);
    almenac.humidity_to_location.translate(humidity)
}
