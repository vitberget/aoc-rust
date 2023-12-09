use std::collections::BTreeMap;

use anyhow::Context;

pub type Network = BTreeMap<Location, (Location, Location)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Location(char,char,char);

impl Location {
    pub(crate) fn new(word: &str) -> anyhow::Result<Self> {
        let mut chars = word.chars();
        Ok(Self(chars.next().context("Missing first char")?, 
                chars.next().context("Missing second char")?, 
                chars.next().context("Missing third char")? ))
    }

    pub(crate) fn ends_with(&self, letter: char) -> bool {
        self.2 == letter
    }
}
