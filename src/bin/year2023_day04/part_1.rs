use crate::structs::Card;

pub fn solve_part_1(cards: &[Card]) -> anyhow::Result<u32> {
    Ok(cards.iter()
       .map(score_card)
       .sum())
}

fn score_card(card: &Card) -> u32 {
    match card.get_intersecting() {
        0 => 0,
        n => 2u32.pow(n-1)
    }
}
