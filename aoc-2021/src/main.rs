use crate::challenge::ChallengeSolver;
use crate::day1::solver::Day1Solver;
use crate::day2::solver::Day2Solver;

pub mod challenge;

mod day1;
mod day2;
mod day3;


fn main() {
    let day1 = Day1Solver::parse("inputs/day1.txt".into());
    let day1_solution = day1.solve_all();
    println!("day1:\nPart 1: {}\nPart 2: {}", day1_solution.0, day1_solution.1);

    let day2 = Day2Solver::parse("inputs/day2.txt".into());
    let day2_solution = day2.solve_all();
    println!("day2:\nPart 1: {}\nPart 2: {}", day2_solution.0, day2_solution.1);
}
