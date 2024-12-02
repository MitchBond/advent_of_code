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


struct Report {
    levels: Vec<i32>
}

impl Report {
    fn diffs(&self) -> Vec<i32> {
        let mut diffs = Vec::new();
        let without_first = &self.levels[1..];
        let without_last = &self.levels[..(self.levels.len() - 1)];
        for (x, next_x) in without_last.iter().zip(without_first.iter()){
            diffs.push(next_x - x);
        }
        return diffs
    }
    fn is_valid(&self) -> bool {
        let diffs = self.diffs();
        let strictly_increasing = diffs.iter().all(|x| x > &0);
        let strictly_decreasing = diffs.iter().all(|x| x < &0);
        let max_diff = diffs.iter().all(|x| x.abs() <= 3);
        return (strictly_increasing || strictly_decreasing) && max_diff;
    }

    fn is_valid_2(&self) -> bool {
        if self.is_valid() {
            return true;
        } else {
            for i in 0..self.levels.len() {
                let mut sublist = self.levels.clone();
                sublist.remove(i);
                let report = Report {levels : sublist};
                if report.is_valid(){
                    return true;
                }

            }
            return false;
        }



    }
}


fn parse_input(input: String) -> Vec<Report> {
    let lines = input.split("\n").map(
        |line| line.split(" ").map(|num| num.parse::<i32>().unwrap())
    );

    return lines.map(|level| Report { levels: level.collect() }).collect()
}


fn part_1(input: &Vec<Report>) -> usize { 
    let valid_reports: Vec<&Report> = input.iter().filter(|report| report.is_valid()).collect();
    return valid_reports.len()
}

fn part_2(input: &Vec<Report>) -> usize {
    let valid_reports: Vec<&Report> = input.iter().filter(|report| report.is_valid_2()).collect();
    return valid_reports.len()
}

fn main() {
    let input: String = read_input(false);
    let parsed_input = parse_input(input);
    println!("Part 1: {}", part_1(&parsed_input));
    println!("Part 2: {}", part_2(&parsed_input));
}
