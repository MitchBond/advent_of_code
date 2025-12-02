use std::{cmp::max, collections::HashSet, fs};

fn read_input(is_practice: bool) -> String {
    let file_name: &str = if is_practice {
        "input/practice_input.txt"
    } else {
        "input/full_input.txt"
    };

    let contents = fs::read_to_string(file_name).expect("Should be able to read file");
    return contents;
}

#[derive(Debug)]
struct ProductRange {
    start: i64,
    end: i64,
}

fn p1_invalid_ids_in_range(product_range: &ProductRange) -> HashSet<i64> {
    let mut invalid_ids = HashSet::<i64>::new();
    let start = product_range.start.to_string();
    let end = product_range.end.to_string();
    let first_half_start = if start.len() > 1 {
        start.split_at(start.len() / 2).0
    } else {
        "0"
    };
    let end_midpoint: usize = if end.len() % 2 == 0 {
        end.len() / 2
    } else {
        end.len() / 2 + 1
    };

    let (first_half_end, _) = end.split_at(end_midpoint);
    let first_half_start: i64 = str::parse(first_half_start).unwrap();
    let first_half_end: i64 = str::parse(first_half_end).unwrap();

    for i in first_half_start..(first_half_end + 1) {
        let mut num_string = i.to_string();
        num_string.push_str(&i.to_string());
        let num: i64 = str::parse(&num_string).unwrap();
        if num >= product_range.start && num <= product_range.end {
            invalid_ids.insert(num);
        }
    }

    return invalid_ids;
}

fn p2_invalid_ids_in_range(product_range: &ProductRange) -> HashSet<i64> {
    let mut candidates = HashSet::<String>::new();
    let end_str = product_range.end.to_string();
    let max_length = end_str.len();
    let end_midpoint: usize = if end_str.len() % 2 == 0 {
        end_str.len() / 2
    } else {
        end_str.len() / 2 + 1
    };
    let (first_half_end, _) = end_str.split_at(end_midpoint);
    let first_half_end = str::parse::<i64>(first_half_end).unwrap();

    for i in 1..(first_half_end + 1) {
        let num_string = i.to_string();
        let max_repeats = max((max_length / num_string.len() + 1), 2);
        for rep in 2..(max_repeats + 1) {
            candidates.insert(num_string.repeat(rep));
        }
    }

    let invalid_candidates: HashSet<i64> = candidates
        .iter()
        .map(|v| str::parse::<i64>(v).unwrap())
        .filter(|v| v >= &product_range.start && v <= &product_range.end)
        .collect();

    return invalid_candidates;
}

fn part_1(product_ranges: &[ProductRange]) -> i64 {
    let invalid_ids: Vec<i64> = product_ranges
        .iter()
        .map(|r| p1_invalid_ids_in_range(r).iter().sum())
        .collect();
    return invalid_ids.into_iter().sum();
}

fn part_2(product_ranges: &[ProductRange]) -> i64 {
    let invalid_ids: Vec<HashSet<i64>> = product_ranges
        .iter()
        .map(|r| p2_invalid_ids_in_range(r))
        .collect();
    return invalid_ids.iter().map(|v| v.iter().sum::<i64>()).sum();
}

fn create_product_range_from_line(line: &str) -> ProductRange {
    let (l, r) = line.split_once("-").unwrap();
    return ProductRange {
        start: str::parse(l).unwrap(),
        end: str::parse(r).unwrap(),
    };
}

fn main() {
    let input: String = read_input(false);
    let product_ranges: Vec<ProductRange> = input
        .split(",")
        .map(|l| create_product_range_from_line(l))
        .collect();
    println!("Part 1: {}", part_1(&product_ranges));
    println!("Part 2: {}", part_2(&product_ranges));
}
