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

fn n_adjacent_locations(loc: &(usize, usize), location_set: &HashSet<(usize, usize)>) -> usize {
    let mut adjacent_elements: Vec<(usize, usize)> = vec![(loc.0, loc.1 + 1), (loc.0 + 1, loc.1)];
    if loc.0 > 0 {
        adjacent_elements.push((loc.0 - 1, loc.1));
    }
    if loc.1 > 0 {
        adjacent_elements.push((loc.0, loc.1 - 1));
    }

    let valid_adjacent_elements: Vec<&(usize, usize)> = adjacent_elements
        .iter()
        .filter(|l| location_set.contains(&l))
        .collect();
    return valid_adjacent_elements.len();
}

struct Region {
    locations: HashSet<(usize, usize)>,
}

impl Region {
    fn area(&self) -> usize {
        return self.locations.len();
    }

    fn perimeter(&self) -> usize {
        return self
            .locations
            .iter()
            .map(|l| 4 - n_adjacent_locations(l, &self.locations))
            .sum();
    }

    fn price(&self) -> usize {
        return self.area() * self.perimeter();
    }
}

fn get_total_sides(region: &Region, input_grid: &Vec<Vec<char>>) -> usize {
    return region
        .locations
        .iter()
        .map(|l| vertices(l, input_grid))
        .sum();
}

fn vertices(loc: &(usize, usize), input_grid: &Vec<Vec<char>>) -> usize {
    let c = input_grid[loc.1][loc.0];
    let max_x = input_grid[0].len() - 1;
    let max_y = input_grid.len() - 1;

    let left: bool = if loc.0 > 0 {
        input_grid[loc.1][loc.0 - 1] == c
    } else {
        false
    };

    let right = if loc.0 < max_x {
        input_grid[loc.1][loc.0 + 1] == c
    } else {
        false
    };

    let up = if loc.1 > 0 {
        input_grid[loc.1 - 1][loc.0] == c
    } else {
        false
    };

    let down = if loc.1 < max_y {
        input_grid[loc.1 + 1][loc.0] == c
    } else {
        false
    };

    let left_up = if loc.0 > 0 && loc.1 > 0 {
        input_grid[loc.1 - 1][loc.0 - 1] == c
    } else {
        false
    };

    let left_down = if loc.0 > 0 && loc.1 < max_y {
        input_grid[loc.1 + 1][loc.0 - 1] == c
    } else {
        false
    };

    let right_up = if loc.0 < max_x && loc.1 > 0 {
        input_grid[loc.1 - 1][loc.0 + 1] == c
    } else {
        false
    };

    let right_down = if loc.0 < max_x && loc.1 < max_y {
        input_grid[loc.1 + 1][loc.0 + 1] == c
    } else {
        false
    };

    return get_vertexes(
        &left,
        &left_up,
        &up,
        &right_up,
        &right,
        &right_down,
        &down,
        &left_down,
    );
}

fn get_vertexes(
    left: &bool,
    left_up: &bool,
    up: &bool,
    right_up: &bool,
    right: &bool,
    right_down: &bool,
    down: &bool,
    left_down: &bool,
) -> usize {
    let mut v = 0;
    if !left_up && (left == up) {
        v += 1
    }
    if !right_up && (right == up) {
        v += 1
    }
    if !right_down && (right == down) {
        v += 1
    }
    if !left_down && (left == down) {
        v += 1
    }

    if *left_up && !left && !up {
        v += 1
    }
    if *right_up && !right && !up {
        v += 1
    }
    if *right_down && !right && !down {
        v += 1
    }
    if *left_down && !left && !down {
        v += 1
    }
    return v;
}

fn valid_adjacent_elements(
    loc: &(usize, usize),
    input_grid: &Vec<Vec<char>>,
    val: char,
) -> Vec<(usize, usize)> {
    let mut adjacent_elements: Vec<(usize, usize)> = vec![(loc.0, loc.1 + 1), (loc.0 + 1, loc.1)];
    if loc.0 > 0 {
        adjacent_elements.push((loc.0 - 1, loc.1));
    }
    if loc.1 > 0 {
        adjacent_elements.push((loc.0, loc.1 - 1));
    }
    let grid_size: (usize, usize) = (input_grid[0].len(), input_grid.len());

    return adjacent_elements
        .iter()
        .filter(|(x, y)| x >= &0 && x < &grid_size.0 && y >= &0 && y < &grid_size.1)
        .filter(|(x, y)| input_grid[*y][*x] == val)
        .map(|x| *x)
        .collect();
}

fn find_region(loc: (usize, usize), char_grid: &Vec<Vec<char>>) -> Region {
    let mut locations: HashSet<(usize, usize)> = HashSet::new();
    locations.insert(loc);
    let mut n_locations: usize = locations.len();
    let val = char_grid[loc.1][loc.0];
    let mut adjacent_locations = valid_adjacent_elements(&loc, char_grid, val);
    for l in adjacent_locations {
        locations.insert(l);
    }
    while locations.len() > n_locations {
        n_locations = locations.len();
        adjacent_locations = locations
            .iter()
            .map(|l| valid_adjacent_elements(&l, char_grid, val))
            .flatten()
            .collect();
        for l in adjacent_locations {
            locations.insert(l);
        }
    }
    return Region { locations };
}

fn create_regions(char_grid: &Vec<Vec<char>>) -> Vec<Region> {
    let x_len = char_grid[0].len();
    let y_len = char_grid.len();

    let mut regions: Vec<Region> = Vec::new();

    let mut available_locations: HashSet<(usize, usize)> = HashSet::new();
    for x in 0..x_len {
        for y in 0..y_len {
            available_locations.insert((x, y));
        }
    }

    while available_locations.len() > 0 {
        let loc = available_locations.iter().next().unwrap();
        let new_region = find_region(*loc, char_grid);
        for l in new_region.locations.iter() {
            available_locations.remove(&l);
        }

        regions.push(new_region);
    }
    return regions;
}

fn main() {
    let input: String = read_input(false);
    let char_grid: Vec<Vec<char>> = input.split("\n").map(|s| s.chars().collect()).collect();

    let regions = create_regions(&char_grid);
    let prices: Vec<usize> = regions.iter().map(|r| r.price()).collect();

    println!("Part 1: {}", prices.iter().sum::<usize>());
    println!(
        "Part 2: {}",
        regions
            .iter()
            .map(|r| r.area() * get_total_sides(r, &char_grid))
            .sum::<usize>()
    );
}
