
use super::utils::get_input;

pub fn day1_solve() {
    let input_text = get_input("day1");
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