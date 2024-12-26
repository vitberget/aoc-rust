use std::path::Path;
use std::io::Write as _;
use std::fs::{create_dir, File};

use anyhow::ensure;

pub fn create(year: u16, day:u16) -> anyhow::Result<()> {
    ensure!(year > 2013, "Year must be between 2014-2099");
    ensure!(year < 2100, "Year must be between 2014-2099");
    ensure!(day > 0, "Day must be between 1-25");
    ensure!(day < 26, "Day must be between 1-25");

    const BASE: &str = "aoc/src/bin/";
    
    let base = Path::new(BASE);
    ensure!(base.is_dir(), "{:?} is not a dir", base);

    let new_dir_str = format!("year{year}_day{day:02}");
    let new_dir = base.join(&new_dir_str);
    ensure!(!new_dir.exists(), "{new_dir_str} already exists");
    create_dir(&new_dir)?;

    let mut file = File::create(new_dir.join("main.rs"))?;
    file.write_all(include_bytes!("aoc_main.rs.txt"))?;

    let mut file = File::create(new_dir.join("part1.rs"))?;
    file.write_all(include_bytes!("aoc_part1.rs.txt"))?;
    
    let mut file = File::create(new_dir.join("part2.rs"))?;
    file.write_all(include_bytes!("aoc_part2.rs.txt"))?;
    
    Ok(())
}
