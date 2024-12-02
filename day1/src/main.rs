use std::fs;
use day1::quicksort;

fn main() {
    let puzzle_input = fs::read_to_string("src/puzzle_input.txt").expect("Failed to read puzzle_input.txt");

    let (mut list_1, mut list_2) = divide_input(&puzzle_input);

    if list_1.len() != list_2.len() {
        panic!("Lists are not of equal length.");
    }

    let difference_sum = find_solution_part1(&mut list_1, &mut list_2);

    println!("The sum of the differences between each list item is {difference_sum}");
}

fn find_similarity_score(list_1: &Vec<u32>, list_2: &Vec<u32>) -> u32 {
    
}

fn find_solution_part1(mut list_1: &mut Vec<u32>, mut list_2: &mut Vec<u32>) -> u32 {
    quicksort(&mut list_1);
    quicksort(&mut list_2);

    let mut difference_sum = 0;

    for i in 0..list_1.len() {
        let diff = list_1[i].abs_diff(list_2[i]);
        difference_sum += diff;
    }

    difference_sum
}

fn divide_input(puzzle_input: &String) -> (Vec<u32>, Vec<u32>) {
    let mut list_1 = Vec::<&str>::new();
    let mut list_2 = Vec::<&str>::new();

    let lines = puzzle_input.lines().collect::<Vec<_>>();

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

    let list_1 = list_1.into_iter()
        .map(|item| {
            item.parse::<u32>().expect("Failed to parse &str to u32 in list_1")
        })
        .collect::<Vec<_>>();

    let list_2 = list_2.into_iter()
        .map(|item| {
            item.parse::<u32>().expect("Failed to parse &str into u32 in list_2")
        })
        .collect::<Vec<_>>();

    (list_1, list_2)
}