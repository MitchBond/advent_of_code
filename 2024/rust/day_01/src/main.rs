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

fn get_lists(input: String) -> (Vec<i32>, Vec<i32>) {
    let lines = input.split("\n");
    let parsed_lines: Vec<Vec<&str>> = lines.map(|input_line| input_line.split("   ").collect()).collect();
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    for l in parsed_lines{
        let a = l[0].trim().parse::<i32>().unwrap();
        let b = l[1].trim().parse::<i32>().unwrap();
        left_list.push(a);
        right_list.push(b);
    }
    return (left_list, right_list)
}


fn part_1(left: Vec<i32>, right: Vec<i32>) -> i32 { 
    let mut left_sorted = left.clone();
    left_sorted.sort();
    let mut right_sorted = right.clone();
    right_sorted.sort();

    let zipped_arrays = left_sorted.iter().zip(right_sorted.iter());
    return zipped_arrays.map(|(a, b)| (a-b).abs()).sum();

}

fn part_2(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let right_counts = right.iter().counts();
    return left.iter().map(|x| x * (*right_counts.get(x).unwrap_or(&0) as i32)).sum();
}

fn main() {
    let input: String = read_input(false);
    let (left, right) = get_lists(input);

    println!("Part 1: {}", part_1(left.clone(), right.clone()));
    println!("Part 2: {}", part_2(left.clone(), right.clone()));
}
