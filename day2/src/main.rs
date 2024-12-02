use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("src/puzzle_input.txt").expect("Failed to read puzzle input.");

    let reports = parse_input(&puzzle_input);

    for report in reports {
        println!("{report:?}");
    }
}

fn parse_input(puzzle_input: &str) -> Vec<Vec<i32>> {
    let lines = puzzle_input.lines().collect::<Vec<&str>>();

    lines
        .into_iter()
        .map(|line| {
            line.split(" ")
                .map(|num|
                    num.parse::<i32>()
                    .expect("Failed to convert &str to i32."))
                    .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>()
}