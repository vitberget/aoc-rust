use std::collections::BTreeMap;

use anyhow::Context;
use regex::Regex;

use crate::structs::{Location, Network};

pub fn text_to_desert_map(text: &str) -> anyhow::Result<(String, Network)> {
    let mut lines = text.lines();
    let chars = lines.next().context("No instructions")?.to_string();
    
    lines.next().context("Missing blank linke")?;

    let rx = Regex::new(r"(?P<here>\w{3}) = \((?P<left>\w{3}), (?P<right>\w{3})\)")?;

    let mut network: Network = BTreeMap::new();

    for line in lines {
        let captures = rx.captures(line).context("No captures")?;
        let here = Location::new(&captures["here"])?;
        let left = Location::new(&captures["left"])?;
        let right = Location::new(&captures["right"])?;

        network.insert(here, (left, right));
    }

    Ok((chars, network))
}
