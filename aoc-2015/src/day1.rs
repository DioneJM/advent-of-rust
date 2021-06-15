use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::io::Read;

pub fn day1_solve() {
    let input_text = get_input("/Users/dionemorales/Development/advent-of-rust/aoc-2015/src/inputs/day1.txt");
    if let Ok(input) = input_text {
        day1_part1_solve(&input);
        day1_part2_solve(&input);
    }
}

fn day1_part1_solve(input: &String) {
    let mut level: i32 = 0;
    input.chars().for_each(|char| {
        match char {
            '(' => level = level + 1,
            ')' => level = level - 1,
            c => println!("Invalid character: {}", c)
        }
    });
    println!("Day 1 Part 1 Solution: {}", level);
}

fn day1_part2_solve(input: &String) {
    let mut level: i32 = 0;
    let mut visited_basement: bool = false;
    input.chars().enumerate().for_each(|(idx, char)| {
        match char {
            '(' => level = level + 1,
            ')' => level = level - 1,
            c => println!("Invalid character: {}", c)
        }
        if !visited_basement && level < 0 {
            println!("Day 1 Part 2 Solution: {}", idx + 1);
            visited_basement = true;
        }
    });
}

fn get_input(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(Path::new(file_path))?;
    Ok(contents)
}