use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Card {
    pub id: u32,
    pub winning: HashSet<u8>,
    pub player: HashSet<u8>,
}

impl Card {
    pub fn get_intersecting(&self) -> u32 {
        HashSet::intersection(&self.winning, &self.player).count() as u32
    }
}
