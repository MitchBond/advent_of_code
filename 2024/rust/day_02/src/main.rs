use std::fs;

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

    fn diffs(levels: &Vec<i32>) -> Vec<i32> {
        let mut diffs = Vec::new();
        let without_first = &levels[1..];
        let without_last = &levels[..(levels.len() - 1)];
        for (x, next_x) in without_last.iter().zip(without_first.iter()) {
            diffs.push(next_x - x);
        }
        return diffs;
    }
    fn is_valid(levels: &Vec<i32>) -> bool {
        let diffs = diffs(&levels);
        let strictly_increasing = diffs.iter().all(|x| x > &0);
        let strictly_decreasing = diffs.iter().all(|x| x < &0);
        let max_diff = diffs.iter().all(|x| x.abs() <= 3);
        return (strictly_increasing || strictly_decreasing) && max_diff;
    }

    fn is_valid_2(levels: &Vec<i32>) -> bool {
        if is_valid(&levels) {
            return true;
        } else {
            for i in 0..levels.len() {
                let mut sublist = levels.clone();
                sublist.remove(i);
                if is_valid(&sublist) {
                    return true;
                }
            }
            return false;
        }
    }


fn parse_input(input: String) -> Vec<Vec<i32>> {
    return input
        .split("\n")
        .map(|line| line.split(" ").map(|num| num.parse::<i32>().unwrap()).collect()).collect();

}

fn part_1(input: &Vec<Vec<i32>>) -> usize {
    let valid_reports: Vec<&Vec<i32>> = input.iter().filter(|report| is_valid(&report)).collect();
    return valid_reports.len();
}

fn part_2(input: &Vec<Vec<i32>>) -> usize {
    let valid_reports: Vec<&Vec<i32>> = input.iter().filter(|report| is_valid_2(&report)).collect();
    return valid_reports.len();
}

fn main() {
    let input: String = read_input(false);
    let parsed_input = parse_input(input);
    println!("Part 1: {}", part_1(&parsed_input));
    println!("Part 2: {}", part_2(&parsed_input));
}
