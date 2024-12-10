use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub(super) struct SleighLaunchSafetyManual {
    pub(super) page_ordering_rules: HashMap<u64, HashSet<u64>>,
    pub(super) pages: Vec<Vec<u64>>
}

impl From<&str> for SleighLaunchSafetyManual {
    fn from(text: &str) -> Self {
        let mut lines = text.lines();
        let mut page_ordering_rules: HashMap<u64, HashSet<u64>> = HashMap::new();
        while let Some(line) = lines.by_ref().next() {
            if line.is_empty() { break }
            let mut parts = line.split("|");
            let left = parts.next().unwrap_or("0").parse::<u64>().unwrap_or(0);
            let right = parts.next().unwrap_or("0").parse::<u64>().unwrap_or(0);

            match page_ordering_rules.get_mut(&left) {
                Some(set) => { set.insert(right); },
                None => {
                    let mut set: HashSet<u64> = HashSet::new();
                    let _ = set.insert(right);
                    let _ = page_ordering_rules.insert(left, set);
                }
            }
        }

        let pages: Vec<Vec<u64>> = lines
            .map(|line| line.split(",")
                .map(|word| word.parse().unwrap())
                .collect())
            .collect();

        SleighLaunchSafetyManual { page_ordering_rules, pages }
    }
}
