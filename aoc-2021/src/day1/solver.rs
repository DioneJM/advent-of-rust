use std::cmp::{max};
use std::fs;
use crate::challenge::ChallengeSolver;

pub struct Day1Solver {
    input: String,
}

impl Day1Solver {
    pub fn parse(input_file: String) -> Self {
        let base_path = std::env::current_dir().expect("Current directory not found");
        let file_path = base_path.join(&input_file);
        let input = fs::read_to_string(file_path).expect("Failed to read file");
        Day1Solver {
            input
        }
    }

    fn get_measurements(&self) -> Vec<i64> {
        self.input.lines().map(|line| {
            let measurement: i64 = line.trim().parse::<i64>().unwrap();
            measurement
        }).collect()
    }

    fn calculate_measurement_increase(&self, measurements: Vec<i64>) -> i64 {
        let mut count = 0;
        let mut previous_measurement = None;
        for measurement in measurements {
            match previous_measurement {
                Some(previous) => {
                    if measurement > previous {
                        count = count + 1;
                    }
                }
                None => ()
            }
            previous_measurement = Some(measurement);
        }
        count
    }
}

impl ChallengeSolver for Day1Solver {
    fn solve_part_one(&self) -> String {
        self.calculate_measurement_increase(self.get_measurements()).to_string()
    }

    fn solve_part_two(&self) -> String {
        let mut window_measurements: Vec<i64> = vec![];
        let measurements = self.get_measurements();
        for (idx, current) in measurements.iter().enumerate() {
            if idx != 0 {
                let previous = measurements.get(max((idx as i32) - 1, 0) as usize);
                let next = measurements.get(idx + 1);
                if previous.is_some() && next.is_some() {
                    let previous = previous.cloned().unwrap();
                    let current: i64 = current.clone();
                    let next = next.cloned().unwrap();
                    let sum = previous + current + next;
                    window_measurements.push(sum);
                }
            }
        }
        self.calculate_measurement_increase(window_measurements).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::{ChallengeSolver, Day1Solver};

    #[test]
    fn example_1_part_1() {
        let input = r#"199
                      200
                      208
                      210
                      200
                      207
                      240
                      269
                      260
                      263"#.to_string();
        let solver = Day1Solver {
            input
        };
        let answer = solver.solve_part_one();
        assert_eq!(answer.parse::<i32>().unwrap(), 7)
    }

    #[test]
    fn example_1_part_2() {
        let input = r#"199
                      200
                      208
                      210
                      200
                      207
                      240
                      269
                      260
                      263"#.to_string();
        let solver = Day1Solver {
            input
        };
        let answer = solver.solve_part_two();
        assert_eq!(answer.parse::<i32>().unwrap(), 5)
    }
}
