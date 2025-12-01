use std::fs;

fn read_input(is_practice: bool) -> String {
    let file_name: String = if is_practice {
        "input/practice_input.txt"
    } else {
        "input/full_input.txt"
    }.to_string();
    
    let contents = fs::read_to_string(file_name).expect("Should be able to read file");
    return contents
}


fn part_1(instructions: &Vec<Instruction>) -> i32 { 
    let mut pos: i32 = 50;
    let mut zero_count: i32 = 0;

    for instruction in instructions {
        pos += instruction.direction * instruction.distance;
        pos = pos.rem_euclid(100);
        
        if pos == 0 {
            zero_count += 1
        }
    }
    return zero_count
}


fn part_2(instructions: &Vec<Instruction>) -> i32 {
    let mut pos: i32 = 50;
    let mut zero_count: i32 = 0;

    for instruction in instructions {
        let dist_from_zero = if instruction.direction == -1 && pos > 0{
        100 - pos
    } else {
        pos
    };
        let num_zero_passed = (dist_from_zero + instruction.distance).div_euclid(100);
        zero_count += num_zero_passed;

        pos += instruction.direction * instruction.distance;
        pos = pos.rem_euclid(100);
        
    }
    return zero_count
}

struct Instruction {
    direction: i32,
    distance: i32,
}


fn create_instruction_from_line(line: &str) -> Instruction {
    let (a, b) = line.split_at(1);
    let direction = match a {
        "L" => -1,
        "R" => 1,
        _ => panic!("Direction should be one of L / R"),
    };
    let distance = b.parse::<i32>().expect("Distance should be an integer");
    Instruction { direction, distance }
}

fn main() {
    let input: String = read_input(false);
    let instructions: Vec<Instruction> = input.split("\n").map(|l| create_instruction_from_line(l)).collect();
    
    println!("Part 1: {}", part_1(&instructions));
    println!("Part 2: {}", part_2(&instructions));
}