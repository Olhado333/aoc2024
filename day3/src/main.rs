use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("src/puzzle_input.txt").expect("Failed to read in puzzle input.");
    println!("{puzzle_input}");
}
