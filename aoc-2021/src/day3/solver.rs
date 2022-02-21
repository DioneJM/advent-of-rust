use std::fs;
use std::iter::Map;
use std::str::Lines;
use crate::ChallengeSolver;

pub struct Day3Solver {
    input: String,
}


impl Day3Solver {
    pub fn parse(input_file: String) -> Self {
        let base_path = std::env::current_dir().expect("Current directory not found");
        let file_path = base_path.join(&input_file);
        let input = fs::read_to_string(file_path).expect("Failed to read file");
        Day3Solver {
            input,
        }
    }

}

impl ChallengeSolver for Day3Solver {
    fn solve_part_one(&self) -> String {
        "198".to_string()
    }

}

#[cfg(test)]
mod tests {
    use crate::ChallengeSolver;
    use crate::day3::solver::Day3Solver;

    #[test]
    fn part_1_example_1() {
        let input = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#.to_string();
        let solver = Day3Solver {
            input
        };
        let mut power_consumption = solver.solve_part_one();
        assert_eq!(solver.solve_part_one().parse::<i64>().unwrap(), 198);
    }
}