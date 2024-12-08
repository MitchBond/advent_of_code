use std::{
    collections::HashSet,
    fs,
    ops::{Add, Sub},
};

use itertools::Itertools;

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

#[derive(Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn is_in_grid(&self, grid_size: &Coord) -> bool {
        return (self.x >= 0) && (self.y >= 0) && (self.x < grid_size.x) && (self.y < grid_size.y)
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn get_antinodes(v1: &Coord, v2: &Coord) -> Vec<Coord> {
    let v1 = *v1;
    let v2 = *v2;
    let dist: Coord = v2.clone() - v1.clone();
    return vec![v1 - dist.clone(), v2 + dist.clone()];
}

fn part_1(antennae: &Vec<(char, Coord)>, grid_size: Coord) -> usize {
    let mut sorted_antennae = antennae.clone();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    sorted_antennae.sort_by(|(c1, _), (c2, _)| c1.cmp(c2));

    // chunking required sorting
    let chunked_antennae = sorted_antennae.iter().chunk_by(|(c, _coord)| c);
    for (_key, chunk) in &chunked_antennae {
        let chunk_coords: Vec<Coord> = chunk.map(|(_c, coord)| *coord).collect();
        let pairs: Vec<Vec<&Coord>> = chunk_coords.iter().combinations(2).collect();

        let new_antinodes = pairs
            .iter()
            .map(|v| get_antinodes(v[0], v[1]))
            .flatten()
            .filter(|c| c.is_in_grid(&grid_size));

        for a in new_antinodes {
            antinodes.insert((a.x, a.y));
        }
    }

    return antinodes.len();
}


fn get_antinodes_2(v1: &Coord, v2: &Coord, grid_size: &Coord) -> Vec<Coord> {
    let mut antinodes: Vec<Coord> = Vec::new();
    let v1 = *v1;
    let v2 = *v2;
    antinodes.push(v1);
    antinodes.push(v2);
    let dist: Coord = v2.clone() - v1.clone();

    let mut a1 = v1 - dist.clone();
    let mut a2 = v2 + dist.clone();

    while a1.is_in_grid(grid_size) {
        antinodes.push(a1);
        a1 = a1 - dist.clone();
    }

    while a2.is_in_grid(grid_size) {
        antinodes.push(a2);
        a2 = a2 + dist.clone();
    }

    return antinodes;
}

fn part_2(antennae: &Vec<(char, Coord)>, grid_size: Coord) -> usize {
    let mut sorted_antennae = antennae.clone();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    sorted_antennae.sort_by(|(c1, _), (c2, _)| c1.cmp(c2));
    // chunking required sorting
    let chunked_antennae = sorted_antennae.iter().chunk_by(|(c, _coord)| c);
    for (_key, chunk) in &chunked_antennae {
        let chunk_coords: Vec<Coord> = chunk.map(|(_c, coord)| *coord).collect();
        let pairs: Vec<Vec<&Coord>> = chunk_coords.iter().combinations(2).collect();

        let new_antinodes = pairs
            .iter()
            .map(|v| get_antinodes_2(v[0], v[1], &grid_size))
            .flatten()
            .filter(|c| c.is_in_grid(&grid_size));

        for a in new_antinodes {
            antinodes.insert((a.x, a.y));
        }
    }

    return antinodes.len();
}

fn main() {
    let input: String = read_input(false);
    let grid: Vec<Vec<char>> = input.split("\n").map(|s| s.chars().collect()).collect();

    let grid_size: Coord = Coord {
        x: grid[0].len() as i32,
        y: grid.len() as i32,
    };

    let mut antennae: Vec<(char, Coord)> = Vec::new();

    for x in (0..grid_size.x) {
        for y in (0..grid_size.y) {
            let c = grid[y as usize][x as usize];
            if c != '.' {
                let ant = (c, Coord { x, y });
                antennae.push(ant);
            }
        }
    }

    println!("Part 1: {}", part_1(&antennae, grid_size));
    println!("Part 2: {}", part_2(&antennae, grid_size));
}
