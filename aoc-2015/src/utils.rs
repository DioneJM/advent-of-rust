use std::error::Error;
use std::fs;
use std::path::Path;

pub fn get_input(file_name: &str) -> Result<String, Box<dyn Error>> {
    let base_path: &str = "/Users/dionemorales/Development/advent-of-rust/aoc-2015/src/inputs/";
    let input_file_extension = ".txt";
    let mut file_path: String = base_path.to_owned();
    file_path.push_str(file_name);
    file_path.push_str(input_file_extension);
    let contents = fs::read_to_string(Path::new(file_path.as_str()))?;
    Ok(contents)
}