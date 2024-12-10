use std::collections::{HashMap, HashSet};

use crate::common::SleighLaunchSafetyManual;

pub(crate) fn part1(sleigh_launch_safety_manual: &SleighLaunchSafetyManual) -> anyhow::Result<u64> {
    Ok(sleigh_launch_safety_manual.pages.iter()
        .filter(|page|is_page_sorted(page, &sleigh_launch_safety_manual.page_ordering_rules))
        .map(|page| get_middle_value(page))
        .sum())
}

pub(crate) fn is_page_sorted(page: &[u64], page_ordering_rules: &HashMap<u64, HashSet<u64>>) -> bool {
    let mut page = page.to_vec();

    while !page.is_empty() {
        let left = page.remove(0);
        let wrong_order = page.iter().any(|right| page_ordering_rules.get(right).unwrap().contains(&left));
        if wrong_order { return false }
    }

    true
}

pub(crate) fn get_middle_value(page: &[u64]) -> u64 {
    let idx = page.len() / 2;
    page[idx]
}
