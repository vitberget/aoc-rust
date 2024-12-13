use crate::parse::Arcade;

const UNIT_CONVERSION_ERROR: i128 = 10000000000000;

pub fn part_2(text: &str) -> i128 {
    let mut arcade: Arcade = text.into();

    arcade.adjust_conversion_error(UNIT_CONVERSION_ERROR);

    arcade.press_buttons().into_iter()
        .flatten()
        .map(|button_presses| button_presses.total_button_presses())
        .sum()
}
