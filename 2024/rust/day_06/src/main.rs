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

fn rotate_90(direction: &(i32, i32)) -> (i32, i32) {
    return (-direction.1, direction.0);
}

fn is_on_boundary(loc: &(i32, i32), grid_size: &(i32, i32)) -> bool {
    return loc.0 == 0
        || loc.1 == 0
        || (loc.0 == (grid_size.0 - 1))
        || (loc.1 == (grid_size.1 - 1));
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

fn is_cycle_if_turn_here(
    loc: &(i32, i32),
    direction: &(i32, i32),
    grid: &Vec<Vec<char>>,
    grid_size: &(i32, i32),
) -> bool {
    let mut new_grid = grid.clone();
    new_grid[(loc.1 + direction.1) as usize][(loc.0 + direction.0) as usize] = '#';
    let mut temp_loc = loc.clone();
    let mut temp_direction = direction.clone();
    let mut hash_direction_set: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    while !is_on_boundary(&temp_loc, grid_size) {
        hash_direction_set.insert((temp_loc.0, temp_loc.1, temp_direction.0, temp_direction.1));
        if new_grid[(temp_loc.1 + temp_direction.1) as usize]
            [(temp_loc.0 + temp_direction.0) as usize]
            == '#'
        {
            temp_direction = rotate_90(&temp_direction);
        }
        temp_loc = (temp_loc.0 + temp_direction.0, temp_loc.1 + temp_direction.1);
        if (temp_loc == *loc) && (temp_direction == *direction) {
            return true;
        }

        if hash_direction_set.contains(&(
            temp_loc.0,
            temp_loc.1,
            temp_direction.0,
            temp_direction.1,
        )) {
            return true;
        }
    }

    return false;
}

fn part_2(start_loc: &(usize, usize), grid: &Vec<Vec<char>>, grid_size: &(i32, i32)) -> i32 {
    // Obstacle location can be considered if not the start point or not already visited
    let mut direction: (i32, i32) = (0, -1);
    let mut loc: (i32, i32) = (start_loc.0 as i32, start_loc.1 as i32);
    let start_loc_i32 = loc.clone();
    let mut locations_visited: HashSet<(i32, i32)> = HashSet::new();
    locations_visited.insert(start_loc_i32);
    let mut cycle_points = 0;
    let mut i: i32 = 0;
    while !is_on_boundary(&loc, grid_size) {
        if grid[(loc.1 + direction.1) as usize][(loc.0 + direction.0) as usize] == '#' {
            direction = rotate_90(&direction);
        }
        let potential_obstacle_loc = (loc.0 + direction.0, loc.1 + direction.1);
        let is_valid = ((potential_obstacle_loc != start_loc_i32)
            && !locations_visited.contains(&potential_obstacle_loc)
            && grid[(loc.1 + direction.1) as usize][(loc.0 + direction.0) as usize] != '#');
        println!("Location: {}, is_valid: {}", i, is_valid);
        if is_valid {
            if is_cycle_if_turn_here(&loc, &direction, grid, grid_size) {
                println!(
                    "Found cycle point with obstacle at {} {}",
                    potential_obstacle_loc.0, potential_obstacle_loc.1
                );
                cycle_points += 1
            }
        }
        loc = (loc.0 + direction.0, loc.1 + direction.1);
        locations_visited.insert(loc);
        i += 1;
    }
    return cycle_points;
}

fn main() {
    let input: String = read_input(false);
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();
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

    println!("Part 1: {}", part_1(&start_loc, &grid, &grid_size));

    println!("Part 2: {}", part_2(&start_loc, &grid, &grid_size));
}
