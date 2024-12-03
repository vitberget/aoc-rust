use std::str::FromStr;

pub(crate) fn text_to_vecs(text: &str) -> (Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>) {
    let mut r1_lower: Vec<i32> = vec![];
    let mut r1_upper: Vec<i32> = vec![];
    let mut r2_lower: Vec<i32> = vec![];
    let mut r2_upper: Vec<i32> = vec![];

    for line in text.lines() {
        let parts: Vec<i32> = line.split(',')
            .flat_map(|part| part.split('-'))
            .map(|v| <i32 as FromStr>::from_str(v).expect("i hate input"))
            .collect();

        r1_lower.push(parts[0]);
        r1_upper.push(parts[1]);
        r2_lower.push(parts[2]);
        r2_upper.push(parts[3]);
    }

    (r1_lower, r1_upper, r2_lower, r2_upper)
}
