use std::{collections::btree_set::Difference, fs};

fn main() {
    let puzzle_input =
        fs::read_to_string("src/puzzle_input.txt").expect("Failed to read puzzle input.");

    let reports = parse_input(&puzzle_input);

    println!("The number of safe reports is: {}", num_safe_reports(&reports));

}

fn num_safe_reports(reports: &Vec<Vec<i32>>) -> u32 {
    let mut count = 0;

    for report in reports {
        if is_report_safe_varients(report) {
            count += 1;
        }
    }

    count
}

fn is_report_safe_varients(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let mut varient = report.clone();
        varient.remove(i);

        if is_report_safe(&varient) {
            return true;
        }
    }

    false
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut is_increasing = report[0] < report[1];

    let mut first_pointer = 0;
    let mut second_pointer = 1;

    for _ in 0..(report.len() - 1) {
        let first = report[first_pointer];
        let second = report[second_pointer];
        let current_is_increasing = first < second;

        if current_is_increasing != is_increasing {
            return false;
        } else {
            is_increasing = current_is_increasing;
        }

        let difference = first.abs_diff(second);

        if (difference < 1) || (difference > 3) {
            return false;
        }

        first_pointer = second_pointer;
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