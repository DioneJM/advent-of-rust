use crate::challenge::ChallengeSolver;
use crate::day1::solver::Day1Solver;

mod day1;
pub mod challenge;

fn main() {
    let day1 = Day1Solver::new("inputs/day1.txt".into());

    day1.solve_part_one();
}
