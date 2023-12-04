use std::collections::BTreeMap;

use crate::structs::Card;

#[derive(Debug)]
struct CardWithScratchcards {
    pub id: u32,
    pub scratchcards: Vec<u32>
}

pub fn solve_part_2(cards: &[Card]) -> anyhow::Result<u32> {
    let cards_with_scratchcards: Vec<CardWithScratchcards> = cards.iter()
        .map(card_to_card_with_scratchpads)
        .collect();

    let mut card_count: u32 = 0;
    let mut extra_card_count: BTreeMap<u32,u32> = BTreeMap::new();

    for card in cards_with_scratchcards {
        let this_card_count = 1 + extra_card_count.get(&card.id).unwrap_or(&0);
        card_count += this_card_count;
        card.scratchcards.iter()
            .for_each(|num| {
                match extra_card_count.get_mut(num) {
                    Some(count) => *count += this_card_count,
                    None => {extra_card_count.insert(*num, this_card_count);}
                }
            });
    }

    Ok(card_count)
}

fn card_to_card_with_scratchpads(card: &Card) -> CardWithScratchcards {
    let scratchcards: Vec<u32> = (0..card.get_intersecting()).map(|n| n+1+card.id).collect();
    CardWithScratchcards { id: card.id, scratchcards }
}

#[cfg(test)]
mod tests {
    use crate::parse::parse;
    use super::solve_part_2;

    #[test]
    fn test_with_example() -> anyhow::Result<()> {
        let cards = parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")?;
        let result = solve_part_2(&cards)?;
        assert_eq!(result, 30);
       
        Ok(())
    }
}
