use regex::Regex;
use std::{fs, i32};

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

fn part_1(input: &String) -> i32 {
    let re = Regex::new(r"mul\((?<first>\d*),(?<last>\d*)\)").unwrap();
    return re.captures_iter(&input).map(|groups| &groups["first"].parse::<i32>().unwrap() * &groups["last"].parse::<i32>().unwrap()).sum();
}


fn part_2(input: &String) -> i32 {
    let mut new_input: String = "do()".to_string();
    new_input.push_str(&input);
    let mult_re = Regex::new(r"mul\((?<first>\d*),(?<last>\d*)\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let matches = mult_re.find_iter(&new_input);
    let dos: Vec<usize> = do_re.find_iter(&new_input).map(|m| m.start()).collect(); 
    let donts: Vec<usize> = dont_re.find_iter(&new_input).map(|m| m.start()).collect();

    let valid_matches = matches.filter(|m| {
        let match_start = m.start();
        dos.iter().filter(|&x| x < &match_start).max() > donts.iter().filter(|&x| x < &match_start).max()
    });
    let match_strings: Vec<&str> = valid_matches.map(|m| m.as_str()).collect();
    let joined = match_strings.join("-");

    return part_1(&joined);

}

fn main() {
    let input: String = read_input(false);

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
