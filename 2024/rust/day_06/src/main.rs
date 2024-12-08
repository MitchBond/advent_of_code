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

fn get_locations_visited(
    start_loc: &(usize, usize),
    grid: &Vec<Vec<char>>,
    grid_size: &(i32, i32),
) -> HashSet<(i32, i32)> {
    let mut direction: (i32, i32) = (0, -1);
    let mut loc: (i32, i32) = (start_loc.0 as i32, start_loc.1 as i32);
    let mut locations_visited: HashSet<(i32, i32)> = HashSet::new();
    while !is_on_boundary(&loc, grid_size) {
        if grid[(loc.1 + direction.1) as usize][(loc.0 + direction.0) as usize] == '#' {
            direction = rotate_90(&direction);
        }
        loc = (loc.0 + direction.0, loc.1 + direction.1);
        locations_visited.insert(loc);
    }
    return locations_visited;
}

fn is_in_cycle(
    start_loc: &(usize, usize),
    new_grid: Vec<Vec<char>>,
    grid_size: &(i32, i32),
) -> bool {
    let mut direction: (i32, i32) = (0, -1);
    let mut loc = (start_loc.0 as i32, start_loc.1 as i32);
    let mut state: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    state.insert((loc.0, loc.1, direction.0, direction.1));

    while !is_on_boundary(&loc, grid_size) {
        let next_element = new_grid[(loc.1 + direction.1) as usize][(loc.0 + direction.0) as usize];
        if next_element == '#' {
            direction = rotate_90(&direction);
        } else {
            loc = (loc.0 + direction.0, loc.1 + direction.1);
        }

        let current_state = (loc.0, loc.1, direction.0, direction.1);

        if state.contains(&current_state) {
            return true;
        } else {
            state.insert(current_state);
        }
    }

    return false;
}

fn part_2(
    start_loc: &(usize, usize),
    grid: &Vec<Vec<char>>,
    grid_size: &(i32, i32),
    locations_visited: &HashSet<(i32, i32)>,
) -> usize {
    // Obstacle location can be considered if not the start point or not already visited
    let mut n_cycles: usize = 0;
    for l in locations_visited {
        if ((l.0 as usize) != start_loc.0) || ((l.1 as usize) != start_loc.1){

        let mut new_grid = grid.clone();
        new_grid[l.1 as usize][l.0 as usize] = '#';
        let is_cycle_with_new_grid = is_in_cycle(start_loc, new_grid, grid_size);
        if is_cycle_with_new_grid {
            n_cycles += 1;
        }
    }
    }
    return n_cycles;
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

    let locations_visited = get_locations_visited(&start_loc, &grid, &grid_size);

    println!("Part 1: {}", locations_visited.len());

    println!("Part 2: {}", part_2(&start_loc, &grid, &grid_size, &locations_visited));
}
