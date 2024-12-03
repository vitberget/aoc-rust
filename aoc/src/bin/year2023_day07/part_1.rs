use itertools::Itertools;

use crate::structs::{Plays, Card, Play, Hand};

pub fn solve_part_1(plays: &Plays) -> anyhow::Result<u64> {
    Ok(plays.iter()
       .map(|play| {
           let hand = play_to_hand(play);
           (play, hand)
       })
       .sorted_by(|(play_a, hand_a), (play_b, hand_b)| compare(play_a, hand_a, play_b, hand_b))
       .enumerate()
       .map(|(index, (play,_))| (index as u64+1u64) * play.bet)
       .sum())

}

fn compare(play_a: &&Play, hand_a: &Hand, play_b: &&Play, hand_b: &Hand) -> std::cmp::Ordering {
    if hand_a != hand_b {
        hand_b.cmp(hand_a)
    } else if play_b.cards[0] != play_a.cards[0] {
        card_to_value(&play_a.cards[0]).cmp(&card_to_value(&play_b.cards[0]))
    } else if play_b.cards[1] != play_a.cards[1] {
        card_to_value(&play_a.cards[1]).cmp(&card_to_value(&play_b.cards[1]))
    } else if play_b.cards[2] != play_a.cards[2] {
        card_to_value(&play_a.cards[2]).cmp(&card_to_value(&play_b.cards[2]))
    } else if play_b.cards[3] != play_a.cards[3] {
        card_to_value(&play_a.cards[3]).cmp(&card_to_value(&play_b.cards[3]))
    } else {
        card_to_value(&play_a.cards[4]).cmp(&card_to_value(&play_b.cards[4]))
    }
}

fn play_to_hand(play: &Play) -> Hand {
    let counts = play.cards.iter().counts();
    let counts: Vec<&usize> = counts.values()
        .sorted_by(|a,b| b.cmp(a))
        .collect();

    if counts[0] >= &5 {
        Hand::FiveOfAKind
    } else if counts[0] == &4 {
        Hand::FourOfAKind
    } else if counts[0] == &3 && counts[1] == &2 {
        Hand::FullHouse
    } else if counts[0] == &3 {
        Hand::ThreeOfAKind
    } else if counts[0] == &2 && counts[1] == &2 {
        Hand::TwoPairs
    } else if counts[0] == &2 {
        Hand::Pair
    } else {
        Hand::HighCard
    }
}


fn card_to_value(card: &Card) -> u8 {
    match card {
        Card::Ace => 14,
        Card::King => 13,
        Card::Queen => 12,
        Card::Jack => 11,
        Card::C10 => 10,
        Card::C9 => 9,
        Card::C8 => 8,
        Card::C7 => 7,
        Card::C6 => 6,
        Card::C5 => 5,
        Card::C4 => 4,
        Card::C3 => 3,
        Card::C2 => 2,
    }
}
