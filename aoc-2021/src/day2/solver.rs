use std::fs;
use std::iter::Map;
use std::str::Lines;
use crate::ChallengeSolver;

enum Instruction {
    Forward(i64),
    Down(i64),
    Up(i64),
}

pub struct Day2Solver {
    input: String,
}

type InstructionList<'a> = Map<Lines<'a>, fn(&str) -> Instruction>;

fn parse_instructions(instructions: &String) ->  InstructionList {
    instructions.lines().map(|line| {
        let mut split_line = line.split(' ');
        let (direction, unit) = (
            split_line.next().expect("No direction found"),
            split_line.next().expect("No unit found"),
        );
        let unit = unit.parse::<i64>().unwrap();
        match direction {
            "forward" => Instruction::Forward(unit),
            "down" => Instruction::Down(unit),
            "up" => Instruction::Up(unit),
            _ => panic!("Instruction not supported: {}", line)
        }
    })
}

impl Day2Solver {
    pub fn parse(input_file: String) -> Self {
        let base_path = std::env::current_dir().expect("Current directory not found");
        let file_path = base_path.join(&input_file);
        let input = fs::read_to_string(file_path).expect("Failed to read file");
        Day2Solver {
            input,
        }
    }

    fn calculate_course(instructions: InstructionList) -> (i64, i64) {
        let mut horizontal_position = 0;
        let mut depth = 0;
        instructions.for_each(|instruction| {
            match instruction {
                Instruction::Up(unit) => depth = depth - unit,
                Instruction::Down(unit) => depth = depth + unit,
                Instruction::Forward(unit) => horizontal_position = horizontal_position + unit,
            }
        });
        (horizontal_position,depth)
    }

    fn calculate_course_part_2(instructions: InstructionList) -> (i64, i64) {
        let mut horizontal_position = 0;
        let mut depth = 0;
        let mut aim = 0;
        instructions.for_each(|instruction| {
            match instruction {
                Instruction::Down(unit) => aim = aim + unit,
                Instruction::Up(unit) => aim = aim - unit,
                Instruction::Forward(unit) => {
                    horizontal_position = horizontal_position + unit;
                    depth = depth + aim * unit;
                },
            }
        });
        (horizontal_position,depth)
    }

}

impl ChallengeSolver for Day2Solver {
    fn solve_part_one(&self) -> String {
        let instructions = parse_instructions(&self.input);
        let (horizontal, depth) = Day2Solver::calculate_course(instructions);
        let area = horizontal * depth;
        area.to_string()
    }

    fn solve_part_two(&self) -> String {
        let instructions = parse_instructions(&self.input);
        let (horizontal, depth) = Day2Solver::calculate_course_part_2(instructions);
        let area = horizontal * depth;
        area.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::ChallengeSolver;
    use crate::day2::solver::Day2Solver;

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

    #[test]
    fn part_2_example_1() {
        let input = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#.to_string();
        let solver = Day2Solver {
            input
        };
        assert_eq!(solver.solve_part_two().parse::<i64>().unwrap(), 900)
    }
}