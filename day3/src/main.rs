use std::fs;
use regex::Regex;

fn main() {
    let puzzle_input = fs::read_to_string("src/puzzle_input.txt").unwrap();
    
    let answer = part_one(&puzzle_input);
    println!("{answer}");
}

fn part_one(puzzle_input: &str) -> u32 {
    let pattern = Regex::new(r"mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\)").unwrap();
    let mut sum = 0;

    for (_, [first, second]) in pattern.captures_iter(&puzzle_input).map(|c| c.extract()) {
        let num1 = first.parse::<u32>().unwrap();
        let num2 = second.parse::<u32>().unwrap();

        sum += num1 * num2;
    }
    
    sum
}


