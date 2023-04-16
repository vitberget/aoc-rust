use std::collections::{HashMap, LinkedList};

pub(crate) fn folder_sizes(text: &str) -> Vec<u32> {
    let mut dir_sizes: HashMap<LinkedList<String>, u32> = HashMap::new();
    let mut cwd: LinkedList<String> = LinkedList::new();

    for line in text.lines() {
        match line {
            line if line == "$ ls" => {}
            line if line.starts_with("dir ") => {}

            line if line.starts_with("$ cd ") => cd(&mut cwd, line),
            _ => file(&mut dir_sizes, &cwd, line),
        }
    }

    let mut result: Vec<u32> = vec![];
    dir_sizes.values().for_each(|n| result.insert(0, *n));
    return result;
}

fn cd(cwd: &mut LinkedList<String>, line: &str) {
    let dir = line.split_whitespace().nth(2).unwrap().to_string();
    if dir == ".." {
        cwd.pop_front();
    } else {
        cwd.push_front(dir.clone());
    }
}

fn file(dir_sizes: &mut HashMap<LinkedList<String>, u32>, cwd: &LinkedList<String>, line: &str) {
    let size: u32 = line.split_whitespace().nth(0).unwrap().parse().unwrap();

    let mut key = cwd.clone();
    while !key.is_empty() {
        if let Some(v) = dir_sizes.get_mut(&key) {
            *v += size;
        } else {
            dir_sizes.insert(key.clone(), size);
        }
        key.pop_front();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folder_sizes_ex() {
        let sizes = folder_sizes(include_str!("../../../examples/year2022_day7.txt"));
        assert!(sizes.contains(&584));
        assert!(sizes.contains(&94853));
        assert!(sizes.contains(&24933642));
        assert!(sizes.contains(&48381165));
    }

}