use std::collections::HashSet;

use anyhow::Context;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct Requirements {
    pub width: i64,
    pub height: i64, 

    pub req_0: usize,
    pub req_1: usize,
    pub req_2: usize,
    pub req_3: usize,
    pub req_4: usize,
    pub req_5: usize,
}

impl Requirements {
    pub fn get_border(&self) -> HashSet<(i64,i64)> {
        let mut border: HashSet<(i64,i64)> = HashSet::new();
        for x in -1_i64 .. (self.width + 1) {
            border.insert((x, -1));
            border.insert((x, self.height));
        }
        for y in -1_i64 .. (self.height + 1) {
            border.insert((-1, y));
            border.insert((self.width, y));
        }

        border
    }
}

impl TryFrom<&str> for Requirements {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let regex = Regex::new(r"^(?<width>\d+)x(?<height>\d+): (?<req_0>\d+) (?<req_1>\d+) (?<req_2>\d+) (?<req_3>\d+) (?<req_4>\d+) (?<req_5>\d+)$")?;
        let captures = regex.captures(line).context("No captures")?;

        let width: i64 = captures.name("width").context("No group width")?.as_str().parse()?;
        let height: i64 = captures.name("height").context("No group height")?.as_str().parse()?;
        let req_0: usize = captures.name("req_0").context("No group req_0")?.as_str().parse()?;
        let req_1: usize = captures.name("req_1").context("No group req_1")?.as_str().parse()?;
        let req_2: usize = captures.name("req_2").context("No group req_2")?.as_str().parse()?;
        let req_3: usize = captures.name("req_3").context("No group req_3")?.as_str().parse()?;
        let req_4: usize = captures.name("req_4").context("No group req_4")?.as_str().parse()?;
        let req_5: usize = captures.name("req_5").context("No group req_5")?.as_str().parse()?;

        Ok(Self { width, height, req_0, req_1, req_2, req_3, req_4, req_5 })
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_parse_requirements_1() -> anyhow::Result<()> {
        let reqs: Requirements = "4x4: 0 0 0 0 2 0".try_into()?;    

        assert_eq!(reqs, Requirements { 
            width: 4,
            height: 4,
            req_0: 0,
            req_1: 0,
            req_2: 0,
            req_3: 0,
            req_4: 2,
            req_5: 0 });

        Ok(())
    }

    #[test]
    fn test_parse_requirements_2() -> anyhow::Result<()> {
        let reqs: Requirements = "12x5: 1 0 1 0 3 2".try_into()?;    

        assert_eq!(reqs, Requirements { 
            width: 12,
            height: 5,
            req_0: 1,
            req_1: 0,
            req_2: 1,
            req_3: 0,
            req_4: 3,
            req_5: 2 });

        Ok(())
    }

    #[test]
    fn test_border_4x4() -> anyhow::Result<()> {
        let reqs: Requirements = "4x4: 0 0 0 0 2 0".try_into()?;    
        let border = reqs.get_border();

        println!("borders: {:?}", border.iter().sorted().collect_vec());

        assert_eq!(border.len(), 4*2 + 4*2 + 4);

        Ok(())
    }
    #[test]
    fn test_border_12x5() -> anyhow::Result<()> {
        let reqs: Requirements = "12x5: 0 0 0 0 2 0".try_into()?;    
        let border = reqs.get_border();

        println!("borders: {:?}", border.iter().sorted().collect_vec());

        assert_eq!(border.len(), 12*2 + 5*2 + 4);

        Ok(())
    }
}
