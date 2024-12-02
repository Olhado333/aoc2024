use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("src/puzzle_input.txt").expect("Failed to read puzzle_input.txt");

    let lines = puzzle_input.lines().collect::<Vec<_>>();
    
    let mut list_1 = Vec::<&str>::new();
    let mut list_2 = Vec::<&str>::new();

    lines.into_iter().for_each(|line| {
        let mut line_split = line.split("   ");
        
        if let Some(first) = line_split.next() {
            list_1.push(first);
        } else {
            panic!("Failed to obtain first half of line.");
        }

        if let Some(second) = line_split.next() {
            list_2.push(second);
        } else {
            panic!("Failed to obtain second half of line.");
        }
    });

    if list_1.len() != list_2.len() {
        panic!("Lists are not of equal length.");
    }

    for i in 0..list_1.len() {
        println!("At {}\nlist_1: {}, list_2: {}.", i, list_1[i], list_2[i]);
    }
}
