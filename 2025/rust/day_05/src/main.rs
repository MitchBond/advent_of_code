use std::{
    cmp::{max, min},
    collections::HashSet,
    fs,
};


fn read_input(is_practice: bool) -> String {
    let file_name: &str = if is_practice {
        "input/practice_input.txt"
    } else {
        "input/full_input.txt"
    };

    return fs::read_to_string(file_name).expect("Should be able to read file");
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct ProductRange {
    start: i64,
    stop: i64,
}

fn process_ranges(ranges: &str) -> Vec<ProductRange> {
    return ranges
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .map(|(start, stop)| ProductRange {
            start: str::parse(start).unwrap(),
            stop: str::parse(stop).unwrap(),
        })
        .collect();
}

fn is_overlapping(a: &ProductRange, b: &ProductRange) -> bool {
    return (b.start >= a.start && b.start <= a.stop) || (b.stop >= a.start && b.stop <= a.stop);
}

fn collapse_overlapping_ranges(ranges: &Vec<ProductRange>) -> HashSet<ProductRange> {
    let mut product_ranges: Vec<ProductRange> = ranges.clone();
    let mut disjoint_ranges: HashSet<ProductRange> = HashSet::new();

    disjoint_ranges.insert(product_ranges.pop().unwrap());
    while !product_ranges.is_empty() {
        let r = product_ranges.pop().unwrap();
        let overlapping_ranges: Vec<ProductRange> = disjoint_ranges
            .iter()
            .filter(|other| is_overlapping(&r, &other) || is_overlapping(&other, &r))
            .cloned()
            .collect();
        if !overlapping_ranges.is_empty() {
            let start = overlapping_ranges.iter().map(|r| r.start).min().unwrap();
            let stop = overlapping_ranges
                .iter()
                .map(|r: &ProductRange| r.stop)
                .max()
                .unwrap();
            let new_range = ProductRange {
                start: min(r.start, start),
                stop: max(r.stop, stop),
            };
            for old_range in overlapping_ranges {
                disjoint_ranges.remove(&old_range);
            }
            disjoint_ranges.insert(new_range);
        } else {
            disjoint_ranges.insert(r);
        }
    }

    return disjoint_ranges;
}

fn main() {
    let input: String = read_input(false);
    let (ranges_str, ingredients_str) = input.split_once("\n\n").unwrap();
    let ranges = process_ranges(ranges_str);
    let ingredients: Vec<i64> = ingredients_str
        .lines()
        .map(|l| str::parse(l).unwrap())
        .collect();

    let valid_ingredients: Vec<i64> = ingredients
        .into_iter()
        .filter(|x| ranges.iter().any(|r| x >= &r.start && x <= &r.stop))
        .collect();

    let collapsed_ranges = collapse_overlapping_ranges(&ranges);

    println!("n_valid_ingredients: {}", valid_ingredients.len());
    println!(
        "n_valid ids: {}",
        collapsed_ranges
            .iter()
            .map(|r| r.stop - r.start + 1)
            .sum::<i64>()
    );
}
