use std::fs;

fn main() {
    let puzzle_input =
        fs::read_to_string("src/puzzle_input.txt").expect("Failed to read puzzle input.");

    let reports = parse_input(&puzzle_input);

    println!("The number of safe reports is: {}", num_safe_reports(&reports));
}

fn num_safe_reports(reports: &Vec<Vec<i32>>) -> u32 {
    let mut count = 0;

    for report in reports {
        if find_safety(report) {
            count += 1;
        }
    }

    count
}

fn find_safety(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        return true;
    }

    let mut first = 0;
    let mut second = 1;
    let increasing = report[first] < report[second];

    for _ in 0..(report.len() - 1) {
        if (report[first] < report[second]) != increasing {
            return false;
        }

        let difference = report[first].abs_diff(report[second]);

        if (difference < 1) || (difference > 3) {
            return false;
        }

        first += 1;
        second += 1;
    }

    true
}

fn parse_input(puzzle_input: &str) -> Vec<Vec<i32>> {
    let lines = puzzle_input.lines().collect::<Vec<&str>>();

    lines
        .into_iter()
        .map(|line| {
            line.split(" ")
                .map(|num| num.parse::<i32>().expect("Failed to convert &str to i32."))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>()
}