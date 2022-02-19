use std::fs;
use std::path::Path;
use crate::challenge::ChallengeSolver;

pub struct Day1Solver {
    input: String
}

impl Day1Solver {
    pub fn new(input_file: String) -> Self {
        let base_path = std::env::current_dir().expect("Current directory not found");
        let file_path = base_path.join(&input_file);
        let input = fs::read_to_string(file_path).expect("Failed to read file");
        Day1Solver {
            input
        }
    }
}

impl ChallengeSolver for Day1Solver {
    fn solve_part_one(&self) -> String {
        let contents = &self.input;
        contents.into()
    }

    fn solve_part_two(&self) -> String {
        todo!()
    }
}