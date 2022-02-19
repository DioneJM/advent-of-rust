use crate::challenge::ChallengeSolver;
use crate::day1::solver::Day1Solver;

mod day1;
pub mod challenge;

fn main() {
    let day1 = Day1Solver::parse("inputs/day1.txt".into());
    let day1_solution = day1.solve_all();

    println!("day1:\nPart 1: {}\nPart 2: {}", day1_solution.0, day1_solution.1);
}
