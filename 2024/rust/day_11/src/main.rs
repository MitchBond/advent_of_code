use std::{collections::{HashMap, HashSet}, fs};
use itertools::{concat, Itertools};

fn read_input(is_practice: bool) -> String {
    let file_name: String = if is_practice {
        "input/practice_input.txt"
    } else {
        "input/full_input.txt"
    }.to_string();
    
    let contents = fs::read_to_string(file_name).expect("Should be able to read file");
    return contents
}


fn update(num: &i128) -> Vec<i128> {
    if *num == 0 {
        return vec![1];
    } 

    let num_string = num.to_string();
    if num_string.len().rem_euclid(2) == 0 {
        let (num1, num2) = num_string.split_at(num_string.len() / 2);
        return vec![num1.parse::<i128>().unwrap(), num2.parse::<i128>().unwrap()];
    }
    let new_val: i128 = num * 2024;
    return vec![new_val];
}


fn part_1(stones: &Vec<i128>) -> usize {
    let new_stones =  stones.clone();
    let mut stone_counts: HashMap<i128, usize> = HashMap::new();
    for x in new_stones {
        *stone_counts.entry(x).or_default() += 1;
    }
    for _ in 0..25 {
        stone_counts = update_counts(stone_counts);

    }
    return stone_counts.values().sum::<usize>()
}

fn update_counts(current_counts: HashMap<i128, usize>) -> HashMap<i128, usize> {
    let mut new_counts: HashMap<i128, usize> = HashMap::new();
    for (x, count) in current_counts.iter() {
        let new_vals = update(x);
        for v in new_vals {
            *new_counts.entry(v).or_default() += count;
        }
    }
    return new_counts;
}

fn part_2(stones: &Vec<i128>) -> usize {
    let new_stones =  stones.clone();
    let mut stone_counts: HashMap<i128, usize> = HashMap::new();
    for x in new_stones {
        *stone_counts.entry(x).or_default() += 1;
    }
    for _ in 0..75 {
        stone_counts = update_counts(stone_counts);

    }
    return stone_counts.values().sum::<usize>()
}

fn main() {
    let input: String = read_input(false);
    let stones: Vec<i128> = input.split(" ").map(|s| s.parse::<i128>().unwrap()).collect();
    println!("Part 1: {}", part_1(&stones));
    println!("Part 2: {}", part_2(&stones));
}
