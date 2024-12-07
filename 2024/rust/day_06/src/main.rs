use std::{collections::HashSet, fs};
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


fn is_on_boundary(loc: &(i32, i32), grid_size: &(i32, i32)) -> bool {
    return loc.0 == 0 || loc.1 == 0 || (loc.0 == (grid_size.0 - 1)) || (loc.1 == (grid_size.1 - 1));
}


fn part_1(start_loc: &(usize, usize), grid: &Vec<Vec<char>>, grid_size: &(i32, i32)) -> usize { 
    let mut direction: (i32, i32) = (0, -1);
    let mut loc: (i32, i32) = (start_loc.0 as i32, start_loc.1 as i32);
    let mut locations_visited: HashSet<(i32, i32)> = HashSet::new();
    while !is_on_boundary(&loc, grid_size) {
        if grid[(loc.1 + direction.1) as usize][(loc.0 + direction.0) as usize] == '#' {
            direction = (-direction.1, direction.0);
        }
        loc = (loc.0 + direction.0, loc.1 + direction.1);
        locations_visited.insert(loc);
    }
    return locations_visited.len();
}

fn part_2(obstacles: Vec<(usize, usize)>) -> usize {
    // Being forced to turn at a point would create a cycle if 3rd turn is in line with current position
    return 5
}

fn main() {
    let input: String = read_input(false);
    let grid: Vec<Vec<char>> = input.split("\n").map(|s| s.chars().collect::<Vec<char>>()).collect();
    let grid_size: (i32, i32) = (grid[0].len() as i32, grid.len() as i32);
    let mut obstacles: Vec<(usize, usize)> = Vec::new();
    let mut start_loc: (usize, usize) = (0, 0);
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '^' {
                start_loc = (x, y);
            }
            if *c == '#' {
                obstacles.push((x, y));
            }
        }
    }

    println!("Start Loc: {} {}", start_loc.0, start_loc.1);
    println!("-----");
    obstacles.iter().for_each(|(x, y)| println!("x={}, y={}", x, y));
    println!("Part 1: {}", part_1(&start_loc, &grid, &grid_size));
    println!("Part 2: {}", part_2());
}
