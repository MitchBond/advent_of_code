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

fn can_be_solved(target: &i64, vals: &Vec<i64>) -> bool {
    let mut totals: Vec<i64> = Vec::new();
    totals.push(vals[0]);
    let remaining_vals: Vec<i64> = vals[1..].to_vec();
    for x in remaining_vals {
        let mut mult: Vec<i64> = totals.iter().map(|t| t * x).filter(|t| t <= &target).collect();
        totals = totals.iter().map(|t| t + x).filter(|t| t <= &target).collect();
        totals.append(& mut mult); 
        if totals.len() == 0 {
            return false;
        }
    }
    return totals.iter().any(|t| t == target);
}

fn part_1(input: &Vec<(i64, Vec<i64>)>) -> i64 { 
    let solveable = input.iter().filter(|(target, vals)| can_be_solved(target, vals));

    return solveable.map(|x| x.0).sum();
}

fn concat(a: &i64, b: &i64) -> i64 {
    let mut full_str = a.to_string();
    let b_str = b.to_string();
    full_str.push_str(&b_str);
    return full_str.parse::<i64>().unwrap();
}


fn can_be_solved_2(target: &i64, vals: &Vec<i64>) -> bool {
    let mut totals: Vec<i64> = Vec::new();
    totals.push(vals[0]);
    let remaining_vals: Vec<i64> = vals[1..].to_vec();
    for x in remaining_vals {
        let mut mult: Vec<i64> = totals.iter().map(|t| t * x).filter(|t| t <= &target).collect();
        let mut concat: Vec<i64> = totals.iter().map(|t| concat(&t, &x)).filter(|t| t <= &target).collect();
        totals = totals.iter().map(|t| t + x).filter(|t| t <= &target).collect();
        totals.append(& mut mult); 
        totals.append(& mut concat);
        if totals.len() == 0 {
            return false;
        }
    }
    return totals.iter().any(|t| t == target);
}

fn part_2(input: &Vec<(i64, Vec<i64>)>) -> i64 {
    let solveable = input.iter().filter(|(target, vals)| can_be_solved_2(target, vals));
    return solveable.map(|x| x.0).sum();
}

fn parse_row(s: &str) -> (i64, Vec<i64>) {
    let (target, vals) = s.split_once(": ").unwrap();
    return (target.parse::<i64>().unwrap(), vals.split(" ").map(|x| x.parse::<i64>().unwrap()).collect());
}

fn main() {
    let input: String = read_input(false);
    let parsed_input: Vec<(i64, Vec<i64>)> = input.split("\n").map(|s| parse_row(s)).collect();

    println!("Part 1: {}", part_1(&parsed_input));
    println!("Part 2: {}", part_2(&parsed_input));
}
