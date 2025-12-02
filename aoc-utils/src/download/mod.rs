use std::fs::{read_to_string, File};
use std::io::Write as _;
use std::path::Path;

use anyhow::ensure;
use reqwest::StatusCode;

pub fn download(year: u16, day:u16) -> anyhow::Result<()> {
    ensure!(year > 2013, "Year must be between 2014-2099");
    ensure!(year < 2100, "Year must be between 2014-2099");
    ensure!(day > 0, "Day must be between 1-25");
    ensure!(day < 26, "Day must be between 1-25");
    ensure!(year < 2025 || day < 13, "In 2025 and later, day must be 1-12");

    const BASE: &str = "puzzles";
    let base = Path::new(BASE);
    ensure!(base.is_dir(), "{:?} is not a dir", base);

    let puzzle_str = format!("year{year}_day{day}.txt");
    let puzzle_path = base.join(&puzzle_str);
    ensure!(!puzzle_path.exists(), "{puzzle_str} already exists");

    let session_file = Path::new(".AOC_SESSION");
    ensure!(session_file.exists(), ".AOC_SESSION missing");

    let session_cookie = read_to_string(session_file)?;
    let session_cookie = session_cookie.trim();

    let client = reqwest::blocking::Client::new();
    let res = client.get(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .header("cookie", format!("session={session_cookie}"))
        .send()?;

    ensure!(res.status() == StatusCode::OK, "Failed to download: {}", res.status());

    let mut file = File::create(puzzle_path)?;
    file.write_all(&res.bytes()?)?;

    Ok(())
}
