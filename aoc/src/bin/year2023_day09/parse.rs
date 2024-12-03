pub fn text_to_numbers(text: String) -> Vec<Vec<i128>> {
    text.lines()
        .map(line_to_numbers)
        .collect()
}

fn line_to_numbers(line: &str) -> Vec<i128> {
    line.split_whitespace()
        .map(|word| word.parse().unwrap())
        .collect()
}
