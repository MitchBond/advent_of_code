use std::{collections::HashSet, fs, vec};

fn read_input(is_practice: bool) -> String {
    let file_name: String = if is_practice {
        "input/practice_input.txt"
    } else {
        "input/full_input.txt"
    }.to_string();
    
    let contents = fs::read_to_string(file_name).expect("Should be able to read file");
    return contents
}


fn valid_adjacent_elements(loc: &(usize, usize), input_grid: &Vec<Vec<usize>>, next_val: usize) -> Vec<(usize, usize)> {
    let mut adjacent_elements: Vec<(usize, usize)> = vec![(loc.0, loc.1 + 1), (loc.0 + 1, loc.1)];
    if loc.0 > 0 {
        adjacent_elements.push((loc.0 - 1, loc.1));
    }
    if loc.1 > 0 {
        adjacent_elements.push((loc.0, loc.1 - 1));
    }
    let grid_size: (usize, usize) = (input_grid[0].len(), input_grid.len());

    return adjacent_elements.iter().filter(|(x, y)| x >= &0 && x < &grid_size.0 && y >= &0 && y < &grid_size.1).filter(|(x, y)| input_grid[*y][*x] == next_val).map(|x| *x).collect();
}

fn advance_paths(current_locations: HashSet<(usize, usize)>, input_grid: &Vec<Vec<usize>>, next_val: usize) -> HashSet<(usize, usize)> {
    let mut new_locations: HashSet<(usize, usize)> = HashSet::new();
    for l in current_locations {
        let adjacent = valid_adjacent_elements(&l, input_grid, next_val);
        for new_l in adjacent {
            new_locations.insert(new_l);
        }
    }
    return new_locations;
}

fn location_hash_set(v: &Vec<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    for l in v {
        set.insert(*l);
    }
    return set;
}

fn part_1(start_locations: &Vec<(usize, usize)>, input_grid: &Vec<Vec<usize>>) -> usize { 
    let path_vecs: Vec<Vec<(usize, usize)>> = start_locations.clone().iter().map(|x| vec![*x]).collect();
    let mut paths: Vec<HashSet<(usize, usize)>> = path_vecs.clone().iter().map(|v|location_hash_set(v)).collect();
    for next_val in 1..10 {
        paths = paths.iter().map(|loc_set| advance_paths(loc_set.clone(), input_grid, next_val)).collect();
    }

    return paths.iter().map(|s| s.len()).sum();
}


fn part_2(start_locations: &Vec<(usize, usize)>, input_grid: &Vec<Vec<usize>>) -> usize { 
    let mut paths: Vec<(usize, usize)> = start_locations.clone();
    for next_val in 1..10 {
        paths = paths.iter().map(|loc| valid_adjacent_elements(&loc, input_grid, next_val)).flatten().collect();
    }

    return paths.len();
}


fn main() {
    let input: String = read_input(false);
    let char_grid: Vec<Vec<usize>> = input.split("\n").map(|s| s.chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect()).collect();
    let mut locations: Vec<Vec<(usize, usize)>> = Vec::new();
    for _ in 0..10 {
        locations.push(Vec::new());
    }
    for y in 0..char_grid.len() {
        for x in 0..(char_grid[0].len()) {
            let val = char_grid[y][x];
            if locations[val].len() == 0 {
                locations[val] = vec![(x, y)]
            } else {
                locations[val].push((x, y));
            }
        }
    }
    let start_locations = &locations[0];
    println!("Part 1: {}", part_1(start_locations, &char_grid));
    println!("Part 2: {}", part_2(start_locations, &char_grid));
}
