use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use log::{debug, trace};

fn main() {
    //Set $RUST_LOG='trace' or 'debug' depending on what you want to see
    // https://docs.rs/env_logger/latest/env_logger/#enabling-logging
    env_logger::init();

    trace!("Starting Application!");
    debug!("Debug test!");

    let input_file_handle = File::open("input.txt").expect("Could not open input file!");
    let input_file_lines = BufReader::new(input_file_handle).lines();

    let mut sum_of_priorities = 0;

    for input_line in input_file_lines {
        if let Ok(current_line) = input_line {
            let mut line_chars = current_line.char_indices();
            // The rucksack has equal number of items in both compartments
            // The sum of the number of items in both compartments will always be even
            // Therefore it will be divisible by two.
            trace!("String length: {}", current_line.len());
            let midpoint_index = (current_line.len() / 2) - 1;

            let mut i = 0;

            //Add all known items from first compartment to the HashSet
            // We will use this to find the item in common with the items in the second compartment
            let mut item_set: HashSet<char> = HashSet::new();

            while i <= midpoint_index {
                let (index, current_char) = line_chars.next().unwrap();
                trace!("Index: {} | Character: {}", index, current_char);

                item_set.insert(current_char);

                i += 1;
            }

            debug!("Inserted {} items into HashSet", item_set.len());

            // Go through the second half and find the common item
            'second_while: while i < current_line.len() {
                let (_, current_char) = line_chars.next().unwrap();

                if item_set.contains(&current_char) {
                    //We found the common item. Add its priority to the sum
                    //and break out of while loop

                    let current_char_priority = get_priority(current_char);
                    debug!(
                        "Common item found: {}. Adding priority: {}",
                        current_char, current_char_priority
                    );
                    sum_of_priorities += current_char_priority;
                    break 'second_while;
                }

                i += 1;
            } // while i < current_line.len()
        } else {
            panic!("Could not read input line. Exiting!");
        }
    } // for loop

    println!("Sum of Prioritues: {}", sum_of_priorities);
}

fn get_priority(input: char) -> i32 {
    let lowercase_base_priority = 1;
    let uppercase_base_priority = 27;

    //Handle lowercase
    if input.is_lowercase() {
        let diff = (input as u8) - ('a' as u8);
        return lowercase_base_priority + (diff as i32);
    } else {
        // Handle uppercase
        let diff = (input as u8) - ('A' as u8);
        return uppercase_base_priority + (diff as i32);
    }
}
