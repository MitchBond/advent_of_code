use std::{
    collections::{HashMap, HashSet},
    fs,
};


fn read_input(is_practice: bool) -> String {
    let file_name: &str = if is_practice {
        "input/practice_input.txt"
    } else {
        "input/full_input.txt"
    };

    return fs::read_to_string(file_name).expect("Should be able to read file");
}

type Location = (i64, i64, i64);

fn parse_location(line: &str) -> Location {
    let coords: Vec<i64> = line.split(",").map(|x| str::parse(x).unwrap()).collect();
    if coords.len() == 3 {
        return (coords[0], coords[1], coords[2]);
    } else {
        panic!("Not 3 coordinates in line");
    }
}

fn distance(loc1: &Location, loc2: &Location) -> i64 {
    return (loc1.0 - loc2.0).pow(2) + (loc1.1 - loc2.1).pow(2) + (loc1.2 - loc2.2).pow(2);
}

fn get_parwise_distances(locations: &Vec<Location>) -> Vec<(Location, Location, i64)> {
    let mut distances: Vec<(Location, Location, i64)> = Vec::new();
    for i in 0..locations.len() {
        let l1 = locations[i];
        for j in (i + 1)..locations.len() {
            let l2 = locations[j];
            let dist = distance(&l1, &l2);
            distances.push((l1, l2, dist));
        }
    }

    return distances;
}

fn create_connections_map(
    top_n_connections: &[(Location, Location, i64)],
) -> HashMap<Location, HashSet<Location>> {
    let mut connections: HashMap<Location, HashSet<Location>> = HashMap::new();
    for (a, b, dist) in top_n_connections {
        connections.entry(a.clone()).or_default().insert(b.clone());

        connections.entry(b.clone()).or_default().insert(a.clone());
    }
    return connections;
}

fn get_circuits(connections: HashMap<Location, HashSet<Location>>) -> Vec<HashSet<Location>> {
    let mut connections_mut = connections.clone();
    let mut out: Vec<HashSet<Location>> = Vec::new();
    let mut current_circuit: HashSet<Location> = HashSet::new();

    while !connections_mut.is_empty() {
        if current_circuit.is_empty() {
            let k = connections_mut.keys().next().cloned().unwrap(); // owned key
            let v = connections_mut.remove(&k).unwrap(); // owned value

            current_circuit.insert(k.clone());
            current_circuit.extend(v.into_iter());
        }
        let valid_keys: Vec<&Location> = current_circuit.iter().filter(|k| connections_mut.contains_key(k)).collect();
        if !valid_keys.is_empty() {
            let new_vals: Vec<HashSet<Location>> = valid_keys.iter().map(|k| connections_mut.remove(&k).unwrap()).collect();
            for v in new_vals {
                current_circuit.extend(v);
            }
        } else {
            out.push(current_circuit);
            current_circuit = HashSet::new();
        }
    }
    return out;
}

fn part_2(pairwise_distances: &Vec<(Location, Location, i64)>, locations: &Vec<Location>) -> i64 {
    let mut unseen_locations: HashSet<Location> = HashSet::from_iter(locations.iter().cloned());
    for (a, b, _dist) in pairwise_distances {
        if unseen_locations.len() == 1 {
            let last_loc = unseen_locations.iter().next().unwrap();
            if last_loc == a || last_loc == b {
                return a.0 * b.0;
            }
        }
        unseen_locations.remove(a);
        unseen_locations.remove(b);
    }
    return 0;
}

fn main() {
    let is_practice = false;
    let n_connections = if is_practice { 10 } else { 1000 };
    let input: String = read_input(is_practice);
    let locations: Vec<Location> = input.lines().map(|l| parse_location(l)).collect();
    let mut pairwise_distances = get_parwise_distances(&locations);
    pairwise_distances.sort_by(|a, b| a.2.cmp(&b.2));
    let top_n_connections = &pairwise_distances[..n_connections.min(pairwise_distances.len())];
    let connections: HashMap<Location, HashSet<Location>> =
        create_connections_map(top_n_connections);
    let circuits = get_circuits(connections);
    let mut circuit_lengths: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
    circuit_lengths.sort();
    println!("{:?}", circuit_lengths);
    println!("{:?}", top_n_connections.len());
    println!("{:?}", part_2(&pairwise_distances, &locations));

}
