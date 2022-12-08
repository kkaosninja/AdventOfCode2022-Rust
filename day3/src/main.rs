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

    let input_file_lines = BufReader::new(File::open("input.txt").expect("Could not open input file!")).lines();

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

    debug!("Beginning Part 2 Solution");

    /* Below code strictly for Part 2 Solution */
    let mut sum_of_group_priorities = 0;

    let mut input_file_reader =
        BufReader::new(File::open("input.txt").expect("Could not open input file!"));

    loop {
        let mut first_line = String::new();
        let mut second_line = String::new();
        let mut third_line = String::new();

        // Read the three lines at once into strings.
        // Input file line count should be a multiple of three.
        // So we shouldn't have any problem

        let mut line_read_result = input_file_reader.read_line(&mut first_line);
        if line_read_result.unwrap() == 0 {
            debug!("EOF reached when reading first line of group. Exiting");
            break;
        }

        line_read_result = input_file_reader.read_line(&mut second_line);
        if line_read_result.unwrap() == 0 {
            debug!("EOF reached when reading second line of group. Exiting");
            break;
        }

        line_read_result = input_file_reader.read_line(&mut third_line);
        if line_read_result.unwrap() == 0 {
            debug!("EOF reached when reading third line of group. Exiting");
            break;
        }

        trace!("Line 1: {}", first_line);
        trace!("Line 2: {}", second_line);
        trace!("Line 3: {}", third_line);

        // Break first line into characters aka items. Read them all into the hashset.
        //NOTE:- Don't forget to strip the newline chars. They are included when reading
        let mut first_line_set: HashSet<char> = HashSet::new();
        for first_line_char in first_line.trim().chars() {
            first_line_set.insert(first_line_char);
        }

        let mut second_line_set: HashSet<char> = HashSet::new();
        for second_line_char in second_line.trim().chars() {
            second_line_set.insert(second_line_char);
        }

        // Now we check how many items are common between first and second item sets

        let mut common_items: HashSet<char> = HashSet::new();
        for common_item in first_line_set.intersection(&second_line_set) {
            common_items.insert(common_item.clone());
        }

        //Read in items for third line
        let mut third_line_set: HashSet<char> = HashSet::new();
        for third_line_char in third_line.trim().chars() {
            third_line_set.insert(third_line_char);
        }

        // Now we find out items common to all three lines
        // From the examples, its just one item
        let mut common_items_all: HashSet<char> = HashSet::new();
        for common_item in third_line_set.intersection(&common_items) {
            common_items_all.insert(common_item.clone());
        }

        for item in common_items_all {
            sum_of_group_priorities += get_priority(item);
        }
    }

    println!("Part 1 - Sum of Prioritues: {}", sum_of_priorities);
    println!(
        "Part 2 - Sum of Group Priorities: {}",
        sum_of_group_priorities
    );
}

fn get_priority(input: char) -> i32 {
    let lowercase_base_priority = 1;
    let uppercase_base_priority = 27;

    debug!("Input char is {}", input);

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
