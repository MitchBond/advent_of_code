use regex::Regex;
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

fn parse_input(split_input: &Vec<String>) -> (&Vec<String>, Vec<String>, Vec<String>, Vec<String>) {
    let char_grid: Vec<Vec<char>> = split_input
        .iter()
        .map(|line| line.chars().collect())
        .collect();
    let y_len = split_input.len();
    let x_len = split_input[0].len();
    let mut vertical: Vec<String> = (0..y_len).map(|f| "".to_string()).collect();
    let mut rdiag: Vec<String> = (0..(2 * y_len)).map(|f| "".to_string()).collect();
    let mut ldiag: Vec<String> = (0..(2 * y_len)).map(|f| "".to_string()).collect();
    for i in (0..x_len) {
        for j in (0..y_len) {
            let char = char_grid[i][j];
            vertical[j].push(char);
            let diff: i32 = i32::try_from(i).unwrap() - i32::try_from(j).unwrap();
            let rdiag_idx: usize = usize::try_from(i + j).unwrap();
            let ldiag_idx: usize = usize::try_from((x_len - i) + j).unwrap();
            rdiag[rdiag_idx].push(char);
            ldiag[ldiag_idx].push(char);
        }
    }
    return (split_input, vertical, rdiag, ldiag);
}

fn part_1(
    horizontal: &Vec<String>,
    vertical: &Vec<String>,
    rdiag: &Vec<String>,
    ldiag: &Vec<String>,
) -> usize {
    let xmas_re = Regex::new(r"(XMAS)").unwrap();
    let samx_re = Regex::new(r"(SAMX)").unwrap();
    let horizontal_matches: usize = horizontal
        .iter()
        .map(|s| xmas_re.find_iter(s).count() + samx_re.find_iter(s).count())
        .sum();
    let vertical_matches: usize = vertical
        .iter()
        .map(|s| xmas_re.find_iter(s).count() + samx_re.find_iter(s).count())
        .sum();
    let rdiag_matches: usize = rdiag
        .iter()
        .map(|s| xmas_re.find_iter(s).count() + samx_re.find_iter(s).count())
        .sum();
    let ldiag_matches: usize = ldiag
        .iter()
        .map(|s| xmas_re.find_iter(s).count() + samx_re.find_iter(s).count())
        .sum();
    return ldiag_matches + rdiag_matches + horizontal_matches + vertical_matches;
}

fn is_valid(char_grid: &Vec<Vec<char>>, loc: &(usize, usize)) -> bool {
    let diag1 = HashSet::from([
        char_grid[loc.1 + 1][loc.0 + 1],
        char_grid[loc.1 - 1][loc.0 - 1],
    ]);
    let diag2 = HashSet::from([
        char_grid[loc.1 + 1][loc.0 - 1],
        char_grid[loc.1 - 1][loc.0 + 1],
    ]);

    let desired_set = HashSet::from(['M', 'S']);

    let diag1_valid = diag1 == desired_set;
    let diag2_valid = diag2 == desired_set;

    return diag1_valid && diag2_valid;
}

fn part_2(split_input: &Vec<String>) -> usize {
    let char_grid: Vec<Vec<char>> = split_input.iter().map(|s| s.chars().collect()).collect();
    let a_regex = Regex::new(r"(A)").unwrap();
    let a_locs: Vec<Vec<(usize, usize)>> = split_input
        .iter()
        .enumerate()
        .map(|(y, s)| a_regex.find_iter(s).map(|m| (m.start(), y)).collect())
        .collect();
    let x_max = split_input[0].len();
    let y_max = split_input.len();

    let interior_a: Vec<&(usize, usize)> = a_locs
        .iter()
        .flatten()
        .filter(|(x, y)| x > &0 && y > &0 && x < &(x_max - 1) && y < &(y_max - 1))
        .collect();
    // Avoids going out of bounds when checking diagonal

    return interior_a
        .iter()
        .filter(|x| is_valid(&char_grid, x))
        .count();
}

fn main() {
    let input: String = read_input(false);
    let split_string: Vec<String> = input.split("\n").map(|s| s.to_owned()).collect();
    let (split_string, vertical, rdiag, ldiag) = parse_input(&split_string);
    println!(
        "Part 1: {}",
        part_1(&split_string, &vertical, &rdiag, &ldiag)
    );
    println!("Part 2: {}", part_2(&split_string));
}
