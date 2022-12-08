use std::fs::read_to_string;

use log::debug;

const INPUT_FILENAME: &str = "example.txt";

fn main() {
    env_logger::init();
    debug!("Starting Application!");

    let input_file_lines: Vec<String> = read_to_string(INPUT_FILENAME)
        .expect("Could not read input file!")
        .split('\n')
        .map(|line| String::from(line))
        .collect();

    let input_data_length = input_file_lines[0].len(); // Trees in top and bottom rows
    let input_data_width = input_file_lines.len(); // Ignore the four corner trees already  

    let mut visible_trees = (input_data_length*2) + ((input_data_width*2)-4);

    let mut input_data_vector: Vec<Vec<usize>> = Vec::new();
    
}
