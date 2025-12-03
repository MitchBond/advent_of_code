use std::fs;

fn read_input(is_practice: bool) -> String {
    let file_name: &str = if is_practice {
        "input/practice_input.txt"
    } else {
        "input/full_input.txt"
    };

    let contents = fs::read_to_string(file_name).expect("Should be able to read file");
    return contents;
}

fn get_max_joltage(joltage_ratings: &[i64], n_values: usize) -> i64 {
    let mut max_joltage: i64 = 0;
    let mut left: usize = 0;
    for i in 0..n_values {
        let range_end: usize = joltage_ratings.len() - n_values + i + 1;
        let candidates = &joltage_ratings[left..range_end];
        let max_value = candidates.iter().max().unwrap();
        left += candidates
            .iter()
            .position(|x| x == max_value)
            .unwrap()
            + 1;
        let exponent: u32 = (n_values - i - 1) as u32;
        let multiplier: i64 = 10i64.pow(exponent);
        max_joltage += (*max_value) * multiplier;
    }
    return max_joltage;
}

fn main() {
    let input: String = read_input(false);
    let ratings: Vec<Vec<i64>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| str::parse(&c.to_string()).unwrap())
                .collect()
        })
        .collect();

    let output_joltage: i64 = ratings.iter().map(|row| get_max_joltage(&row, 2)).sum();
    let output_joltage_2: i64 = ratings
        .iter()
        .map(|row| get_max_joltage(&row, 12))
        .sum();
    println!("Part 1: {:?}", output_joltage);
    println!("Part 2: {:?}", output_joltage_2);
}
