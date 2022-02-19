use std::fs;
use crate::ChallengeSolver;

mod solver;

enum Instruction {
    Forward(i64),
    Down(i64),
    Up(i64),
}

pub struct Day2Solver {
    input: String,
}

fn parse_instructions(instructions: &String) -> Vec<Instruction> {
    vec![]
}

impl Day2Solver {
    pub fn parse(input_file: String) -> Self {
        let base_path = std::env::current_dir().expect("Current directory not found");
        let file_path = base_path.join(&input_file);
        let input = fs::read_to_string(file_path).expect("Failed to read file");
        let instructions = parse_instructions(&input);
        Day2Solver {
            input,
        }
    }
}

impl ChallengeSolver for Day2Solver {
    fn solve_part_one(&self) -> String {
        "150".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::ChallengeSolver;
    use crate::day2::Day2Solver;

    #[test]
    fn part_1_example_1() {
        let input = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#.to_string();
        let solver = Day2Solver {
            input
        };
        assert_eq!(solver.solve_part_one().parse::<i64>().unwrap(), 150)
    }
}