#[derive(Debug, Clone)]
pub struct Almenac {
    pub seeds: Vec<i64>,

    pub seed_to_soil: Translations,
    pub soil_to_fertilizer: Translations,
    pub fertilizer_to_water: Translations,
    pub water_to_light: Translations,
    pub light_to_temperature: Translations,
    pub temperature_to_humidity: Translations,
    pub humidity_to_location: Translations,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Translations {
    pub translations: Vec<Translation>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Translation {
    source: i64,
    delta: i64,
    upper_limit: i64,
}

impl Translation {
    pub fn new(source: i64, destination: i64, size: i64) -> Self {
        Self { 
            source,
            upper_limit: source + size,
            delta: destination - source,
        }
    }

    pub fn translate(&self, seed: i64) -> Option<i64> {
        if seed >= self.source && seed < self.upper_limit {
            Some(seed + self.delta)
        } else {
            None
        }
    } 
}

impl Translations {
    pub fn translate(&self, seed: i64) -> i64 {
        for translation in &self.translations {
            if let Some(translated) = translation.translate(seed) {
                return translated;
            }
        }
        seed
    }
}
