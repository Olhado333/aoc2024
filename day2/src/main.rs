use std::fs;

fn main() {
    let puzzle_input =
        fs::read_to_string("src/mini_puzzle_input.txt").expect("Failed to read puzzle input.");

    let reports = parse_input(&puzzle_input);

    println!("The number of safe reports is: {}", num_safe_reports(&reports));
}

fn num_safe_reports(reports: &Vec<Vec<i32>>) -> u32 {
    let mut count = 0;

    for report in reports {
        if is_report_safe(report) {
            count += 1;
            println!("safe");
        }
    }

    count
}

fn problem_dampener(mut report: Vec<i32>) -> Vec<i32> {
    let mut first_pointer = 0;
    let mut second_pointer = 1;

    let increasing = report[first_pointer] < report[second_pointer];

    for _ in 0..(report.len() - 1) {
        if (report[first_pointer] < report[second_pointer]) != increasing {
            report.remove(first_pointer);
            break;
        }

        let difference = report[first_pointer].abs_diff(report[second_pointer]);

        if (difference < 1) || (difference > 3) {
            report.remove(first_pointer);
            break;
        }

        first_pointer += 1;
        second_pointer += 1;
    }

    report
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    println!("Before removing problematic element:\n{report:?}");

    let report = problem_dampener(report.clone());

    println!("After:\n{report:?}");

    if report.len() < 2 {
        return true;
    }

    let mut first_pointer = 0;
    let mut second_pointer = 1;
    let increasing = report[first_pointer] < report[second_pointer];

    for _ in 0..(report.len() - 1) {
        if (report[first_pointer] < report[second_pointer]) != increasing {
            return false;
        }

        let difference = report[first_pointer].abs_diff(report[second_pointer]);

        if (difference < 1) || (difference > 3) {
            return false;
        }

        first_pointer += 1;
        second_pointer += 1;
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