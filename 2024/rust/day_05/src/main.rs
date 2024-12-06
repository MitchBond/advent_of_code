use std::{collections::{HashMap, HashSet}, fs, i32};
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

fn is_valid_update(update: &Vec<i32>, page_ordering: &HashMap<i32, HashSet<i32>>) -> bool {
    // Update is valid if for each element all previous elements don't appear in the map for that element.
    // i.e for each element prior, there is no instruction saying that element needs to be after the current one
    let empty_hash_set: HashSet<i32> = HashSet::new();
    for i in 0..update.len() {
        let current_element: i32 = update[i];
        let prior_elements = &update[0..i].to_vec();
        let prior_set: HashSet<i32> = HashSet::from_iter(prior_elements.iter().cloned());
        let x = page_ordering.get(&current_element).unwrap_or(&empty_hash_set);

        if  !x.is_disjoint(&prior_set) {
            return false
        }
        
    }
    return true;
}


fn part_1(page_ordering: &HashMap<i32, HashSet<i32>>, updates: &Vec<Vec<i32>>) -> i32 { 
    let valid_updates: Vec<&Vec<i32>> = updates.iter().filter(|u| is_valid_update(u, &page_ordering)).collect();
    let middle_values = valid_updates.iter().map(|v| v[v.len().div_ceil(2) - 1]);
    return middle_values.sum();
}


fn reorder_invalid_update(update: &Vec<i32>, page_ordering: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    // Last element in the list must have no instruction saying it must be behind another element in the update
    // This must be the only one where this holds else ordering is ambiguous
    // Remove this and repeat until list is empty and then reverse.
    let empty_hash_set: HashSet<i32> = HashSet::new();
    let update_len: usize = update.len();
    let mut new_update: Vec<i32> = Vec::with_capacity(update_len);
    let mut old_update: Vec<i32> = update.iter().cloned().collect();
    while old_update.len() > 0 {
        let update_hash: HashSet<i32> = HashSet::from_iter(old_update.iter().cloned());

        let elements_ahead = old_update.iter().map(|x| page_ordering.get(x).unwrap_or(&empty_hash_set));
        let is_disjoint = elements_ahead.map(|s| s.is_disjoint(&update_hash));
        for (i, disjoint) in is_disjoint.enumerate() {
            if disjoint {
                // One element has to be disjoint so we don't infinite loop
                new_update.push(old_update[i]);
                old_update.remove(i);
                break;
            }
        }
    }
    new_update.reverse();
    return new_update;
}

fn part_2(page_ordering: &HashMap<i32, HashSet<i32>>, updates: &Vec<Vec<i32>>) -> i32 { 
    let invalid_updates: Vec<&Vec<i32>> = updates.iter().filter(|u| !is_valid_update(u, &page_ordering)).collect();
    let reordered_updates: Vec<Vec<i32>> = invalid_updates.iter().map(|update| reorder_invalid_update(update, &page_ordering)).collect();
    let middle_values = reordered_updates.iter().map(|v| v[v.len().div_ceil(2) - 1]);
    return middle_values.sum();
}

fn create_page_ordering_map(page_ordering: &str) -> HashMap<i32, HashSet<i32>>{
    let pages: Vec<(i32, i32)> = page_ordering.split("\n").map(|s| s.split_once("|").unwrap()).map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())).collect();
    let mut page_order_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    let sorted_pages = pages.into_iter().sorted_by(|a, b| Ord::cmp(&b.0, &a.0));
    let grouped_ordering = sorted_pages.chunk_by(|(k, _v)| *k);
    for (key, chunk) in &grouped_ordering {

        let mapped_to: HashSet<i32> =chunk.map(|(_k, v)| v).collect();
        page_order_map.insert(key, mapped_to);
    }
    return page_order_map;
}

fn create_updates(updates: &str) -> Vec<Vec<i32>> {
    return updates.split("\n").map(|s| s.split(",").map(|n| n.parse::<i32>().unwrap()).collect()).collect();
}

fn main() {
    let input: String = read_input(false);
    let (page_ordering, updates) = input.split_once("\n\n").unwrap();

    let page_order_map = create_page_ordering_map(page_ordering);
    let parsed_updates = create_updates(updates);

    println!("Part 1: {}", part_1(&page_order_map, &parsed_updates));
    println!("Part 2: {}", part_2(&page_order_map, &parsed_updates));
}
