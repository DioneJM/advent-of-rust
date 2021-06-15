mod day1;
mod day2;
mod utils;

use day1::day1_solve;
use day2::day2_solve;

fn main() {
    println!("running advent of rust 2015");
    solve_day1();
    solve_day2();
}

fn solve_day1() {
    println!("Solving day 1...");
    day1_solve();
}

fn solve_day2() {
    println!("Solving day 2...");
    day2_solve();
}

