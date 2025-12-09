use std::fs;

fn read_input(is_practice: bool) -> String {
    let file_name: &str = if is_practice {
        "input/practice_input.txt"
    } else {
        "input/full_input.txt"
    };

    return fs::read_to_string(file_name).expect("Should be able to read file");
}

fn create_sums(numeric_lines: &Vec<&str>) -> Vec<Vec<i64>> {
    let numbers: Vec<Vec<&str>> = numeric_lines
        .iter()
        .map(|l| l.split_whitespace().collect())
        .collect();
    let n_vecs = numbers[0].len();
    let sum_length = numbers.len();

    let mut output: Vec<Vec<i64>> = Vec::new();
    for i in 0..n_vecs {
        let mut single_sum: Vec<i64> = Vec::new();
        for j in 0..sum_length {
            single_sum.push(str::parse(numbers[j][i]).unwrap());
        }
        output.push(single_sum);
    }
    return output;
}

fn create_sums_p2(numeric_lines: &Vec<&str>) -> Vec<Vec<i64>> {
    let line_length = numeric_lines[0].len();
    let line_chars: Vec<Vec<char>> = numeric_lines.iter().map(|l| l.chars().collect()).collect();

    let mut output: Vec<Vec<i64>> = Vec::new();
    let mut single_sum: Vec<i64> = Vec::new();
    for col in (0..line_length) {
        let col_values: Vec<char> = line_chars
            .iter()
            .map(|l| l[col])
            .filter(|c| !c.is_whitespace())
            .collect();
        let all_ws = col_values.is_empty();
        if all_ws {
            output.push(single_sum.clone());
            single_sum = Vec::new();
        } else {
            let num_str: String = col_values.into_iter().collect();
            let num: i64 = str::parse(&num_str).unwrap();
            single_sum.push(num);
        }
    }
    output.push(single_sum);
    return output;
}

fn do_sum(operation: &char, vals: &[i64]) -> i64 {
    if operation == &'+' {
        vals.iter().sum::<i64>()
    } else if operation == &'*' {
        vals.iter().product()
    } else {
        panic!("Unsupported Operation")
    }
}

fn main() {
    let input: String = read_input(false);
    let mut lines: Vec<&str> = input.split("\n").collect();

    let operations: Vec<char> = lines
        .pop()
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let sums = create_sums(&lines);
    let sums_p2 = create_sums_p2(&lines);

    let sum_values: Vec<i64> = operations
        .iter()
        .zip(sums)
        .map(|(op, nums)| do_sum(op, &nums))
        .collect();

    let sum_values_p2: Vec<i64> = operations
        .iter()
        .zip(sums_p2)
        .map(|(op, nums)| do_sum(op, &nums))
        .collect();

    println!("Part 1: {:?}", sum_values.iter().sum::<i64>());
    println!("Part 2: {:?}", sum_values_p2.iter().sum::<i64>());
}
