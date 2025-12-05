use std::{collections::HashSet, fs};

fn read_input(is_practice: bool) -> String {
    let file_name: &str = if is_practice {
        "input/practice_input.txt"
    } else {
        "input/full_input.txt"
    };

    return fs::read_to_string(file_name).expect("Should be able to read file");
}

fn get_adjacent_positions(x: i32, y: i32) -> Vec<(i32, i32)> {
    let mut adjacent_positions: Vec<(i32, i32)> = Vec::new();
    for xdelta in [-1, 0, 1] {
        for ydelta in [-1, 0, 1] {
            adjacent_positions.push((x + xdelta, y + ydelta));
        }
    }
    return adjacent_positions
        .into_iter()
        .filter(|(xloc, yloc)| !(xloc == &x && yloc == &y))
        .collect();
}

fn count_adjacent_rolls(x: i32, y: i32, roll_locations: &HashSet<(i32, i32)>) -> usize {
    let adjacent_positions = get_adjacent_positions(x, y);
    let adjacent_rolls: Vec<(i32, i32)> = adjacent_positions
        .into_iter()
        .filter(|loc| roll_locations.contains(loc))
        .collect();
    return adjacent_rolls.len();
}

fn find_valid_rolls(roll_locations: &HashSet<(i32, i32)>) -> Vec<(i32, i32)> {
    let accessible_rolls: Vec<(i32, i32)> = roll_locations
        .iter()
        .copied()
        .filter(|loc| count_adjacent_rolls(loc.0, loc.1, roll_locations) < 4)
        .collect();

    return accessible_rolls;
}

fn get_roll_locations(grid: &Vec<Vec<char>>) -> HashSet<(i32, i32)> {
    let mut roll_locations: HashSet<(i32, i32)> = HashSet::new();
    for y in 0..grid.len() {
        let line = &grid[y];
        for x in 0..line.len() {
            if line[x] == '@' {
                roll_locations.insert((x as i32, y as i32));
            }
        }
    }
    return roll_locations;
}

fn part_2(roll_locations: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut rolls = roll_locations.clone();
    let mut accessible_rolls: HashSet<(i32, i32)> = HashSet::new();
    let mut removed_rolls: Vec<(i32, i32)> = find_valid_rolls(&rolls);
    while removed_rolls.len() > 0 {
        for r in removed_rolls {
            rolls.remove(&r);
            accessible_rolls.insert(r);
        }

        removed_rolls = find_valid_rolls(&rolls)
    }
    return accessible_rolls;
}

fn main() {
    let input: String = read_input(false);
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let roll_locations = get_roll_locations(&grid);
    let valid_rolls = find_valid_rolls(&roll_locations);
    println!("n_valid_rolls: {}", valid_rolls.len());

    let part_2_ans = part_2(&roll_locations);
    println!("Part 2: {}", part_2_ans.len());
}
