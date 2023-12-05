use anyhow::bail;

use crate::structs::{Almenac, Translation, Translations};

pub fn parse(text: &str) -> anyhow::Result<Almenac> {
    let mut paragraph = 0;
    let mut seeds: Vec<i64> = vec![];

    let mut seed_to_soil: Vec<Translation> = vec![];
    let mut soil_to_fertilizer: Vec<Translation> = vec![];
    let mut fertilizer_to_water: Vec<Translation> = vec![];
    let mut water_to_light: Vec<Translation> = vec![];
    let mut light_to_temperature: Vec<Translation> = vec![];
    let mut temperature_to_humidity: Vec<Translation> = vec![];
    let mut humidity_to_location: Vec<Translation> = vec![];

    for line in text.lines() {
        match line {
            line if line.is_empty() => {}
            line if line.starts_with("seeds: ") => {seeds = parse_seeds(line);}
            line if line.contains("-to-") => {paragraph += 1;}
            line => {
                match paragraph {
                    0 => {}
                    1 => { seed_to_soil.push(line_to_translation(line)); }
                    2 => { soil_to_fertilizer.push(line_to_translation(line)); }
                    3 => { fertilizer_to_water.push(line_to_translation(line)); }
                    4 => { water_to_light.push(line_to_translation(line)); }
                    5 => { light_to_temperature.push(line_to_translation(line)); }
                    6 => { temperature_to_humidity.push(line_to_translation(line)); }
                    7 => { humidity_to_location.push(line_to_translation(line)); }
                    _ => { bail!("to many paragraphs") }
                }

            }
        }
    }

    Ok(Almenac {
        seeds,
        seed_to_soil: Translations { translations: seed_to_soil },
        soil_to_fertilizer: Translations { translations: soil_to_fertilizer },
        fertilizer_to_water: Translations { translations: fertilizer_to_water },
        water_to_light: Translations { translations: water_to_light },
        light_to_temperature: Translations { translations: light_to_temperature },
        temperature_to_humidity: Translations { translations: temperature_to_humidity },
        humidity_to_location: Translations { translations: humidity_to_location },
    })
}


fn parse_seeds(line: &str) -> Vec<i64> {
    line[6..].split_whitespace().map(|word| word.parse().unwrap_or(0)).collect()
}

fn line_to_translation(line: &str) -> Translation {
    let mut splitted = line.split_whitespace();

    let destination: i64 = splitted.next().unwrap_or("0").parse().unwrap_or(0);
    let source: i64 = splitted.next().unwrap_or("0").parse().unwrap_or(0);
    let size: i64 = splitted.next().unwrap_or("0").parse().unwrap_or(0);

    Translation::new(source, destination, size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let almenac = parse(include_str!("example.txt")).unwrap();
        assert_eq!(almenac.seeds, vec![79,14,55,13]);
        assert_eq!(almenac.seed_to_soil.translations, 
                   vec![ Translation::new(98,50,2), Translation::new(50,52,48), ]);
    }
}
