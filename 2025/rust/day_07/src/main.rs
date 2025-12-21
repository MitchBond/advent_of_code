use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

fn read_input(is_practice: bool) -> String {
    let file_name: &str = if is_practice {
        "input/practice_input.txt"
    } else {
        "input/full_input.txt"
    };

    return fs::read_to_string(file_name).expect("Should be able to read file");
}

fn find_splitters(input: &String) -> Vec<(i32, i32)> {
    let chars: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut splitter_locations: Vec<(i32, i32)> = Vec::new();
    let xmax = chars[0].len();
    let ymax = chars.len();
    for y in 0..ymax {
        for x in 0..xmax {
            if chars[y][x] == '^' {
                splitter_locations.push((x as i32, y as i32));
            }
        }
    }
    return splitter_locations;
}

fn find_start_location(input: &String) -> (i32, i32) {
    let lines: Vec<&str> = input.lines().collect();
    let first_line = lines[0];
    let start_loc = first_line.chars().find_position(|c| c == &'S').unwrap().0;
    return (start_loc as i32, 0);
}

fn find_next_splits(loc: &(i32, i32), splitter_locations: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let left_possibles: Vec<(i32, i32)> = splitter_locations
        .iter()
        .filter(|splitter_loc| splitter_loc.0 == loc.0 - 1 && splitter_loc.1 > loc.1)
        .cloned()
        .collect();
    let right_possibles: Vec<(i32, i32)> = splitter_locations
        .iter()
        .filter(|splitter_loc| splitter_loc.0 == loc.0 + 1 && splitter_loc.1 > loc.1)
        .cloned()
        .collect();
    let mut output: Vec<(i32, i32)> = Vec::new();
    if left_possibles.len() > 0 {
        output.push(left_possibles[0]);
    }
    if right_possibles.len() > 0 {
        output.push(right_possibles[0]);
    }
    return output;
}

fn find_next_splits_p2(loc: &(i32, i32), splitter_locations: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let left_possibles: Vec<(i32, i32)> = splitter_locations
        .iter()
        .filter(|splitter_loc| splitter_loc.0 == loc.0 - 1 && splitter_loc.1 > loc.1)
        .cloned()
        .collect();
    let right_possibles: Vec<(i32, i32)> = splitter_locations
        .iter()
        .filter(|splitter_loc| splitter_loc.0 == loc.0 + 1 && splitter_loc.1 > loc.1)
        .cloned()
        .collect();
    let mut output: Vec<(i32, i32)> = Vec::new();
    if left_possibles.len() > 0 {
        output.push(left_possibles[0]);
    } else {
        output.push((loc.0 - 1, -1));
    }
    if right_possibles.len() > 0 {
        output.push(right_possibles[0]);
    } else { 
        output.push((loc.0 + 1, -1));
    }
    return output;
}

fn find_all_splits(
    start_loc: &(i32, i32),
    splitter_locations: &Vec<(i32, i32)>,
) -> HashSet<(i32, i32)> {
    let mut locations_with_splits: HashSet<(i32, i32)> = HashSet::new();
    let align_with_start: Vec<(i32, i32)> = splitter_locations
        .iter()
        .filter(|l| l.0 == start_loc.0)
        .cloned()
        .collect();
    let first_split = align_with_start.first().unwrap();
    let mut splits: HashSet<(i32, i32)> = HashSet::new();
    splits.insert(*first_split);
    while splits.len() > 0 {
        for s in splits.clone() {
            locations_with_splits.insert(s);
        }
        splits = HashSet::from_iter(
            splits
                .iter()
                .map(|s| find_next_splits(s, splitter_locations))
                .flatten(),
        );
    }

    return locations_with_splits;
}

fn find_new_splits_and_propagate_count(
    loc: &(i32, i32),
    count: &i64,
    splitter_locations: &Vec<(i32, i32)>,
) -> Vec<((i32, i32), i64)> {
    let mut out = Vec::new();
    let next_splits = find_next_splits_p2(loc, splitter_locations);
    if next_splits.is_empty() || loc.1 == -1 {
        out.push((loc.clone(), count.clone()));
    } else {
        out = next_splits
            .iter()
            .map(|l| (l.clone(), count.clone()))
            .collect();
    }
    return out;
}

fn find_total_routes(start_loc: &(i32, i32), splitter_locations: &Vec<(i32, i32)>) -> i64 {
    let mut locations_with_counts: HashMap<(i32, i32), i64> = HashMap::new();
    let align_with_start: Vec<(i32, i32)> = splitter_locations
        .iter()
        .filter(|l| l.0 == start_loc.0)
        .cloned()
        .collect();
    let first_split = align_with_start.first().unwrap();
    locations_with_counts.insert(*first_split, 1);
    let mut any_new_splits = true;

    while any_new_splits {
        let next_split_counts: Vec<((i32, i32), i64)> = locations_with_counts
            .iter()
            .map(|(l, c)| find_new_splits_and_propagate_count(l, c, splitter_locations))
            .flatten()
            .collect();

        let mut new_counts: HashMap<(i32, i32), i64> = HashMap::new();

        for (key, value) in next_split_counts {
            *new_counts.entry(key).or_insert(0) += value;
        }
        any_new_splits = new_counts != locations_with_counts;
        locations_with_counts.drain();
        locations_with_counts.extend(new_counts.clone());
    }
    return locations_with_counts.iter().map(|(_l, c)| c).sum();
}

fn main() {
    let input: String = read_input(false);
    let splitter_locations = find_splitters(&input);
    let start_loc = find_start_location(&input);

    let splits = find_all_splits(&start_loc, &splitter_locations);
    let total_routes = find_total_routes(&start_loc, &splitter_locations);

    println!("n_splits: {:?}", splits.len());
    println!("Total Routes: {:?}", total_routes);
}
