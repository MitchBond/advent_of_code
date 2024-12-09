use itertools::Itertools;
use std::{collections::HashSet, fs};

fn read_input(is_practice: bool) -> String {
    let file_name: String = if is_practice {
        "input/practice_input.txt"
    } else {
        "input/full_input.txt"
    }
    .to_string();

    let contents = fs::read_to_string(file_name).expect("Should be able to read file");
    return contents;
}

fn find_next_end_pointer(input_vector: &Vec<i32>, end_pointer: &usize) -> usize {
    let mut new_end_pointer: usize = end_pointer.clone();
    while input_vector[new_end_pointer] == -1 {
        new_end_pointer -= 1;
    }
    return new_end_pointer;
}

fn part_1(input_vector: &Vec<i32>) -> usize {
    let mut start_pointer: usize = 0;
    let mut end_pointer: usize = input_vector.len() - 1;
    let mut new_vector: Vec<i32> = Vec::new();
    while start_pointer <= end_pointer {
        if input_vector[start_pointer] != -1 {
            new_vector.push(input_vector[start_pointer]);
        } else {
            new_vector.push(input_vector[end_pointer]);
            end_pointer -= 1;
        }
        start_pointer += 1;
        end_pointer = find_next_end_pointer(&input_vector, &end_pointer);
    }
    return new_vector
        .iter()
        .enumerate()
        .map(|(i, x)| i * (*x as usize))
        .sum();
}

fn block_value(start: &usize, length: &usize, value: &i32) -> i64 {
    let range_sum: i64 = (*start..(start + length)).sum::<usize>() as i64;
    return range_sum * (*value as i64);
}

fn part_2(blocks: &Vec<(usize, usize, i32)>, spaces: &HashSet<(usize, usize)>) -> i64 {
    let mut spaces_hash: HashSet<(usize, usize)> = spaces.clone();
    let mut reversed_blocks = blocks.clone();
    reversed_blocks.reverse();
    let mut new_blocks: Vec<(usize, usize, i32)> = Vec::new();

    for b in reversed_blocks {
        let temp_spaces = spaces_hash.clone();
        let spaces_can_fit: Vec<&(usize, usize)> = temp_spaces
            .iter()
            .filter(|s| (s.0 < b.0) && (s.1 >= b.1))
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
            .collect();
        if spaces_can_fit.len() > 0 {
            let space_for_block = spaces_can_fit[0];
            new_blocks.push((space_for_block.0, b.1, b.2)); // New block fits in left of empty space

            spaces_hash.insert((b.0, b.1)); // New space where the old block was

            spaces_hash.remove(space_for_block); // Old space no longer valid
            if space_for_block.1 > b.1 {
                spaces_hash.insert((space_for_block.0 + b.1, space_for_block.1 - b.1));
            }
        } else {
            new_blocks.push(b);
        }
    }

    let parts = new_blocks
        .iter()
        .map(|(start, length, val)| block_value(start, length, val));
    return parts.sum();
}

fn create_subvector(i: usize, x: &i32) -> Vec<i32> {
    let n: usize = *x as usize;
    if i.rem_euclid(2) == 0 {
        let id: i32 = (i / 2) as i32;
        return vec![id; n];
    } else {
        return vec![-1; n];
    }
}

fn create_vec_from_input(input: &String) -> Vec<i32> {
    let input_lengths: Vec<i32> = input
        .chars()
        .map(|c| c.to_string().parse::<i32>().unwrap())
        .collect();
    return input_lengths
        .iter()
        .enumerate()
        .map(|(i, x)| create_subvector(i, x))
        .flatten()
        .collect();
}

fn create_blocks_and_spaces(input: &String) -> (Vec<(usize, usize, i32)>, HashSet<(usize, usize)>) {
    let mut blocks: Vec<(usize, usize, i32)> = Vec::new();
    let mut spaces: HashSet<(usize, usize)> = HashSet::new();

    let mut counter: usize = 0;

    for (i, c) in input.chars().enumerate() {
        let val: usize = c.to_string().parse::<usize>().unwrap();
        if i.rem_euclid(2) == 0 {
            blocks.push((counter, val, (i / 2) as i32));
        } else {
            spaces.insert((counter, val));
        }

        counter += val;
    }

    return (blocks, spaces);
}

fn main() {
    let input: String = read_input(false);

    let input_vector = create_vec_from_input(&input);

    let (blocks, spaces) = create_blocks_and_spaces(&input);

    println!("Part 1: {}", part_1(&input_vector));
    println!("Part 2: {}", part_2(&blocks, &spaces));
}
