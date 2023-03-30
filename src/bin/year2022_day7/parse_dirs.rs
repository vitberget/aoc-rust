use std::str::{Lines, SplitWhitespace};

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}

#[derive(Default, Debug)]
pub(crate) struct Dir {
    name: String,
    dirs: Vec<Dir>,
    files: Vec<File>,
    size: Option<u64>
}

pub(crate) fn parse_text(text: &str) -> Dir {
    let mut lines = text.lines().into_iter();

    let mut container = Dir {
        name: "hello".to_string(),
        ..Dir::default()
    };

    parse_line(&mut container, &mut lines);

    return container.dirs.pop().unwrap();
}

fn parse_line(cwd: &mut Dir, lines: &mut Lines<'_>) {
    let line = lines.next();

    if line.is_some() {
        let mut split = line.unwrap().split_whitespace();
        let first_word = split.next().unwrap();

        match first_word {
            "$" => parse_command(cwd, lines, &mut split),
            "dir" => parse_line(cwd, lines),
            _ => parse_file(cwd, lines, &mut split, first_word)
        }
    }
}

fn parse_file(cwd: &mut Dir, lines: &mut Lines, split: &mut SplitWhitespace, first_word: &str) {
    cwd.files.push(File {
        name: split.next().unwrap().to_string(),
        size: first_word.parse().unwrap_or(0)
    });
    parse_line(cwd, lines);
}

fn parse_command(cwd: &mut Dir, lines: &mut Lines, split: &mut SplitWhitespace) {
    if split.next().unwrap() == "cd" {
        let mut dir = Dir {
            name: split.next().unwrap().to_string(),
            ..Dir::default()
        };

        parse_line(&mut dir, lines);
        cwd.dirs.push(dir);
    } else {
        parse_line(cwd, lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let parsed_dirs = parse_text(include_str!("../../../examples/year2022_day7.txt"));

        println!("result {:?}", parsed_dirs);

        assert_eq!("/", parsed_dirs.name)
    }
}