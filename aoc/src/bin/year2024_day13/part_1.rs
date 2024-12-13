use crate::parse::Arcade;

pub fn part_1(text: &str) -> i128 {
    let arcade: Arcade = text.into();

    arcade.press_buttons().into_iter()
        .filter_map(|option| option.map(|bp| bp.total_button_presses()))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::part_1::part_1;
    use crate::parse::{Arcade, ButtonPresses};

    #[test]
    fn example_part_1() {
        let example = include_str!("example.txt");
        assert_eq!(part_1(example), 480);
    }

    #[test]
    fn example_part_1_steps() {
        let example = include_str!("example.txt");
        let arcade: Arcade = example.into();
    
        let button_presses = arcade.press_buttons();

        assert_eq!(button_presses, vec![
            Some(ButtonPresses { button_a: 80, button_b: 40 }),
            None,
            Some(ButtonPresses { button_a: 38, button_b: 86 }),
            None
        ]);

        let button_counts: Vec<Option<i128>> = button_presses.into_iter()
            .map(|option| option.map(|bp| bp.total_button_presses()))
            .collect();

        assert_eq!(button_counts, vec![Some(280), None, Some(200), None]);

        let final_count: i128 = button_counts.iter()
            .flatten()
            .sum();

        assert_eq!(final_count, 480);
    }


}
