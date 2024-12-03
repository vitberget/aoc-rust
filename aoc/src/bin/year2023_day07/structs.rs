pub type Cards = [Card;5];
#[derive(Hash, Eq, PartialEq)]
#[derive(Debug)]
pub enum Card {
    Ace,
    King,
    Queen,
    Jack,
    C10,
    C9,
    C8,
    C7,
    C6,
    C5,
    C4,
    C3,
    C2,
}

#[derive(Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum Hand {
    FiveOfAKind = 0,
    FourOfAKind = 1,
    FullHouse = 2,
    ThreeOfAKind = 3,
    TwoPairs = 4,
    Pair = 5,
    HighCard = 6
}


pub type Plays = Vec<Play>;
#[derive(Debug)]
pub struct Play {
    pub cards: Cards,
    pub bet: u64
}
