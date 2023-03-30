use std::str::{Lines, SplitWhitespace};

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}

#[derive(Default, Debug)]
struct Dir {
    name: String,
    dirs: Vec<Dir>,
    files: Vec<File>,
}

fn main() {}

fn parse_text(text: &str) -> Dir {
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
        if first_word == "$" {
            parse_command(cwd, lines, &mut split)
        } else if first_word == "dir" {
            println!("dir");
            parse_line(cwd, lines);
        } else {
            parse_file(cwd, lines, &mut split, first_word);
        }
    }
}

fn parse_file(cwd: &mut Dir, lines: &mut Lines, split: &mut SplitWhitespace, first_word: &str) {
    println!("parse file");
    let name = split.next().unwrap().to_string();
    cwd.files.push(File {
        name: name,
        size: first_word.parse().unwrap_or(0)
    });
    parse_line(cwd, lines);
}

fn parse_command(cwd: &mut Dir, lines: &mut Lines, split: &mut SplitWhitespace) {
    let command = split.next().unwrap();
    println!("Command! {}", command);
    if command == "cd" {
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
        let result = parse_text(include_str!("../../../examples/year2022_day7.txt"));

        println!("result {:?}", result);

        assert_eq!("/", result.name)
    }
}