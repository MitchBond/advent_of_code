use regex::Regex;
use std::fs;

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

struct Claw {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl Claw {
    fn solve(&self) -> i64 {
        let num = self.prize.0 * self.a.1 - self.prize.1 * self.a.0;
        let denom = (self.b.0 * self.a.1 - self.b.1 * self.a.0);
        if num.rem_euclid(denom) == 0 {
            let b = num / denom;
            let a_num = self.prize.0 - b * self.b.0;
            if a_num.rem_euclid(self.a.0) == 0 {
                let a = a_num / self.a.0;
                return 3 * a + b;
            }
        }
        return 0;
    }
}

fn parse_input(input: String) -> Vec<Claw> {
    let claws: Vec<&str> = input.split("\n\n").collect();
    let num_re = Regex::new(r"(\d+)").unwrap();
    let claw_nums: Vec<Vec<i64>> = claws
        .iter()
        .map(|s| {
            num_re
                .find_iter(s)
                .map(|x| x.as_str().parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    return claw_nums
        .iter()
        .map(|nums| Claw {
            a: (nums[0], nums[1]),
            b: (nums[2], nums[3]),
            prize: (nums[4], nums[5]),
        })
        .collect();
}

fn part_2_claws(claws: &Vec<Claw>) -> Vec<Claw> {
    return claws
        .iter()
        .map(|c| Claw {
            a: c.a,
            b: c.b,
            prize: (c.prize.0 + 10000000000000, c.prize.1 + 10000000000000),
        })
        .collect();
}

fn main() {
    let input: String = read_input(false);
    let claws: Vec<Claw> = parse_input(input);
    let claws_2 = part_2_claws(&claws);
    let solved: Vec<i64> = claws.iter().map(|c| c.solve()).collect();
    let solved_2: Vec<i64> = claws_2.iter().map(|c| c.solve()).collect();

    let total_tokens: i64 = solved.iter().sum();
    let total_tokens_p2: i64 = solved_2.iter().sum();
    println!("Part 1: {}", total_tokens);
    println!("Part 2: {}", total_tokens_p2);
}
