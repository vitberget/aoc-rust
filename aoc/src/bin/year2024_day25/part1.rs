use aoc_procmacros::aoc_profile;

#[aoc_profile]
pub fn part1(text: &str) -> anyhow::Result<usize> {
    let locks_and_keys = LocksAndKeys::parse(text);
    let r = locks_and_keys.locks.iter()
        .map(|lock| locks_and_keys.keys.iter()
            .filter(|key| key_fits_lock(lock, key))
            .count()
        ).sum();
    Ok(r)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PinHeights {
    pub pins: [u8;5],
}

impl PinHeights {
    pub fn new(pin0:u8, pin1:u8, pin2: u8, pin3:u8, pin4: u8) -> Self {
        Self { pins: [pin0, pin1, pin2, pin3, pin4] }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocksAndKeys {
    pub locks: Vec<PinHeights>,
    pub keys: Vec<PinHeights>,
}

impl LocksAndKeys {
    fn parse(text: &str) -> Self {
        let mut locks: Vec<PinHeights> = vec![];
        let mut keys: Vec<PinHeights> = vec![];

        let mut key: Option<PinHeights> = None;
        let mut lock: Option<PinHeights> = None;
        let mut pin_height: u8 = 0;

        for line in text.lines() {
            if key.is_none() && lock.is_none() {
                if line == "#####" {
                    lock = Some(PinHeights::new(7,7,7,7,7));
                    pin_height = 0;
                } else if line == "....." {
                    key = Some(PinHeights::new(0,0,0,0,0));
                    pin_height = 5;
                }
            } else if line.is_empty() {
                if let Some(key) = key { keys.push(key); }
                if let Some(lock) = lock { locks.push(lock); }
                key = None;
                lock = None;
            } else {
                if let Some(key) = key.as_mut() {
                    for (idx, ch) in line.chars().enumerate() {
                        if ch == '#' {
                            key.pins[idx] = u8::max(key.pins[idx], pin_height);
                        }
                    }
                    pin_height = pin_height.saturating_sub(1);
                }
                if let Some(lock) = lock.as_mut() {
                    for (idx, ch) in line.chars().enumerate() {
                        if ch == '.' {
                            lock.pins[idx] = u8::min(lock.pins[idx], pin_height);
                        }
                    }
                    pin_height += 1;
                }
            }
        }

        if let Some(key) = key { keys.push(key); }
        if let Some(lock) = lock { locks.push(lock); }

        Self { locks, keys }
    }
}

pub fn key_fits_lock(lock: &PinHeights, key: &PinHeights) -> bool {
    for idx in 0..5 {
        if lock.pins[idx] + key.pins[idx] > 5 {
            return false;
        }
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        assert_eq!(part1(include_str!("example.txt"))?, 3);
        Ok(())
    }

    #[test]
    fn test_if_key_fits_lock() {
        // Lock 0,5,3,4,3 and key 5,0,2,1,3: overlap in the last column.
        assert!(! key_fits_lock(&PinHeights::new(0,5,3,4,3), &PinHeights::new(5,0,2,1,3)));
        // Lock 0,5,3,4,3 and key 4,3,4,0,2: overlap in the second column.
        assert!(! key_fits_lock(&PinHeights::new(0,5,3,4,3), &PinHeights::new(4,3,4,0,2)));
        // Lock 0,5,3,4,3 and key 3,0,2,0,1: all columns fit!
        assert!(key_fits_lock(&PinHeights::new(0,5,3,4,3), &PinHeights::new(3,0,2,0,1)));
        // Lock 1,2,0,5,3 and key 5,0,2,1,3: overlap in the first column.
        assert!(! key_fits_lock(&PinHeights::new(1,2,0,5,3), &PinHeights::new(5,0,2,1,3)));
        // Lock 1,2,0,5,3 and key 4,3,4,0,2: all columns fit!
        assert!(key_fits_lock(&PinHeights::new(1,2,0,5,3), &PinHeights::new(4,3,4,0,2)));
        // Lock 1,2,0,5,3 and key 3,0,2,0,1: all columns fit!
        assert!(key_fits_lock(&PinHeights::new(1,2,0,5,3), &PinHeights::new(3,0,2,0,1)));
    }

    #[test]
    fn test_parse_one_lock() {
        let items = LocksAndKeys::parse(include_str!("example_1_lock.txt"));
        assert!(items.keys.is_empty());
        assert!(! items.locks.is_empty());
        assert_eq!(items.locks[0], PinHeights::new(0,5,3,4,3));
    }

    #[test]
    fn test_parse_exaple() {
        let items = LocksAndKeys::parse(include_str!("example.txt"));
        assert_eq!(items.locks, vec![
            PinHeights::new(0,5,3,4,3),
            PinHeights::new(1,2,0,5,3),
        ]);
        assert_eq!(items.keys, vec![
            PinHeights::new(5,0,2,1,3),
            PinHeights::new(4,3,4,0,2),
            PinHeights::new(3,0,2,0,1),
        ]);
    }
}
