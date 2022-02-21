use std::fs;
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
        let binary_numbers = self.input.lines();
        let number_length = &binary_numbers.clone().enumerate().into_iter().next().unwrap().1.len();
        let mut gamma_rate_binary = vec![0; number_length.clone()];
        binary_numbers.for_each(|number| {
            for (idx, character) in number.chars().into_iter().enumerate() {
                let bit_value = character.to_string().parse::<u8>().unwrap();
                match bit_value {
                    1 => gamma_rate_binary[idx] = gamma_rate_binary[idx] + 1,
                    0 => gamma_rate_binary[idx] = gamma_rate_binary[idx] - 1,
                    _ => panic!("invalid bit value detected: {}", bit_value)
                }
            }
        });
        let mut epsilon = String::new();
        let mut gamma_rate = String::new();

        gamma_rate_binary
            .into_iter()
            .for_each(|occurences| {
                if occurences > 0 {
                    gamma_rate.push('1');
                    epsilon.push('0');
                } else if occurences < 0 {
                    gamma_rate.push('0');
                    epsilon.push('1');
                } else {
                    panic!("An even amount of 1's and 0's has been found")
                }
            });

        let gamma_rate = i64::from_str_radix(gamma_rate.as_str(), 2).unwrap();
        // todo figure how to calculate this by just getting the compliment of gamma_rate
        let epsilon_rate = i64::from_str_radix(epsilon.as_str(), 2).unwrap();
        let power_consumption = gamma_rate * epsilon_rate;
        power_consumption.to_string()
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
        assert_eq!(solver.solve_part_one().parse::<i64>().unwrap(), 198);
    }
}