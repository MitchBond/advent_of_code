use std::fs;
use itertools::Itertools;

fn read_input(is_practice: bool) -> String {
    let file_name: String = if is_practice {
        "input/practice_input.txt"
    } else {
        "input/full_input.txt"
    }.to_string();
    
    let contents = fs::read_to_string(file_name).expect("Should be able to read file");
    return contents
}


fn part_1() -> i32 { 
    return 4
}

fn part_2() -> i32 {
    return 5
}

fn main() {
    let input: String = read_input(false);

    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}
