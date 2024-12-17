use std::fs;
use regex::Regex;

fn main() {
    let puzzle_input = fs::read_to_string("src/puzzle_input.txt").unwrap();
    
    let answer = part_two(&puzzle_input);
    println!("{answer}");
}

fn part_one(puzzle_input: &str) -> u32 {
    let pattern = Regex::new(r"mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\)").unwrap();
    let mut sum = 0;

    for (_, [first, second]) in pattern.captures_iter(puzzle_input).map(|c| c.extract()) {
        let num1 = first.parse::<u32>().unwrap();
        let num2 = second.parse::<u32>().unwrap();

        sum += num1 * num2;
    }
    
    sum
}

fn part_two(puzzle_input: &str) -> u32 {
    let mut sum = 0;

    let enablers = get_enablers(puzzle_input);
    let multipliers = get_multipliers(puzzle_input);

    if enablers.len() <= 0 {
        return part_one(puzzle_input);
    }

    let mut enablers = enablers.into_iter();

    let mut enabled = true;
    let mut last_enabler = enablers.next().unwrap();

    for (first, second, index) in multipliers {
        if index >= last_enabler.1 {
            enabled = last_enabler.0;

            if let Some(e) = enablers.next() {
                last_enabler = e;
            }
        }

        if enabled {
            sum += first * second;
        }
    }

    sum
}

fn get_multipliers(puzzle_input: &str) -> Vec<(u32, u32, usize)> {
    let pattern = Regex::new(r"mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\)").unwrap();
    let mut output: Vec<(u32, u32, usize)> = Vec::new();

    for found in pattern.captures_iter(puzzle_input) {
        let mut template = (0,0,0);

        if let Some(m) = found.name("first") {
            template.0 = m.as_str().parse().unwrap();
        }

        if let Some(m) = found.name("second") {
            template.1 = m.as_str().parse().unwrap();
            template.2 = m.end();
        }

        output.push(template);
    }

    output
}

fn get_enablers(puzzle_input: &str) -> Vec<(bool, usize)> {
    let pattern = Regex::new(r"do\(\)|don't\(\)").unwrap();
    let mut output: Vec<(bool, usize)> = Vec::new();

    for found in pattern.captures_iter(puzzle_input) {
        let mut state = true;

        if let Some(m) = found.get(0) {
            if m.as_str() == "don't()" {
                state = false;
            }

            output.push((state, m.end()));
        }
    }

    output
}
