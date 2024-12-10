use std::cmp::Ordering;

use crate::common::SleighLaunchSafetyManual;
use crate::part1;

pub(crate) fn part2(sleigh_launch_safety_manual: &SleighLaunchSafetyManual) -> anyhow::Result<u64> {
    Ok(sleigh_launch_safety_manual.pages.iter()
        .filter(|page| ! part1::is_page_sorted(page, &sleigh_launch_safety_manual.page_ordering_rules))
        .map(|page| sort_page(page, sleigh_launch_safety_manual))
        .map(|page| part1::get_middle_value(&page))
        .sum())
}


fn sort_page(page: &[u64], sleigh_launch_safety_manual: &SleighLaunchSafetyManual) -> Vec<u64> {
    let mut page = page.to_vec();

    page.sort_by(|left, right|
        if sleigh_launch_safety_manual.page_ordering_rules.get(left).unwrap().contains(right) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    );

    page
}
