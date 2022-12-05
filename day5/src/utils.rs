use std::{
    collections::LinkedList,
    fs::File,
    io::{BufRead, BufReader},
};

use log::trace;

pub fn get_num_of_stacks(input_file_name: &str) -> i32 {
    //Let's count the size of the stacks while we are at it.

    let input_file_lines =
        BufReader::new(File::open(input_file_name).expect("Could not open input file!")).lines();

    for input_file_line in input_file_lines {
        if let Ok(current_line) = input_file_line {
            // Find the line which has all the numbers
            // Example: 1 2 3

            if current_line.contains('1') {
                //Split on whitespace, and count the numbers to get the number of stacks
                trace!("Stack numbers line found: {}", current_line);
                return current_line.split_whitespace().collect::<Vec<&str>>().len() as i32;
            } else {
                continue;
            }
        } else {
            panic!("Could not ready input line. Exiting!");
        }
    }

    return 0;
}

pub fn insert_crates_into_stacks(input_file_name: &str, stacks_vector: &mut Vec<LinkedList<char>>) {
    let input_file_lines =
        BufReader::new(File::open(input_file_name).expect("Could not open input file!")).lines();

    for input_file_line in input_file_lines {
        if let Ok(current_line) = input_file_line {
            if current_line.contains('1') {
                continue;
            }
            /* Processing Logic
               Consider the example input [M] [Z] [H] [P] [N] [W] [P] [L] [C]
               Suppose this is a string and we are able to index each character
               then the crate names are at odd place, with three charactes separating them
               // string[1] = 'M' string[5] = 'Z'

               We can use a counter initialized with 1, and increment by 4 to get the next value
               Since all input strings are of the same length, this should work fine.

               This way we can process the input and get the "right" stack to the put the crate on
               even when the column has an empty space

               so in an input string like "[N] [C]    "
               string[1] = 'N', string[5] = 'C', string[9] = ' '
               So we know not to add anything in the third stack
            */

            let mut current_crate_index = 1;
            let mut current_stack_index = 1;
            let mut chars_and_indices = current_line.char_indices();

            while let Some((index, crate_name)) = chars_and_indices.next() {
                if index != current_crate_index {
                    continue;
                }
                trace!("Index => {} Crate Name => {}", index, crate_name);

                if crate_name != ' ' {
                    trace!(
                        "Inserting crate {} into stack {}",
                        crate_name,
                        current_stack_index
                    );

                    stacks_vector
                        .get_mut(current_stack_index - 1)
                        .unwrap()
                        .push_front(crate_name)
                }

                current_crate_index += 4;
                current_stack_index += 1;
            }
        } else {
            panic!("Could not read input lines!");
        }
    }
}

pub fn move_crates(
    stacks_vector: &mut Vec<LinkedList<char>>,
    source_stack_index: usize,
    dest_stack_index: usize,
    move_count: usize,
) {
    let mut move_count = move_count;

    while move_count != 0 {
        let popped_crate = stacks_vector
            .get_mut(source_stack_index - 1)
            .unwrap()
            .pop_back()
            .unwrap();

        trace!(
            "Moving crate {} from stack {} to stack {}",
            popped_crate,
            source_stack_index,
            dest_stack_index
        );

        stacks_vector
            .get_mut(dest_stack_index - 1)
            .unwrap()
            .push_back(popped_crate);

        move_count -= 1;
    }
}

/// To help move multiple crates at once with the CrateMover 9001!
pub fn move_crates_part2(
    stacks_vector: &mut Vec<LinkedList<char>>,
    source_stack_index: usize,
    dest_stack_index: usize,
    move_count: usize,
) {
    let mut move_count = move_count;

    let mut popped_crates_list: LinkedList<char> = LinkedList::new();

    while move_count != 0 {
        let popped_crate = stacks_vector
            .get_mut(source_stack_index - 1)
            .unwrap()
            .pop_back()
            .unwrap();
        popped_crates_list.push_front(popped_crate);
        move_count -= 1;
    }

    trace!("Popped Crates List: {:?}", popped_crates_list);

    // Add crates to new stack in such a way that their order is preserved
    // We will pop from the front of the popped_crates_list to push to the destination stack

    while !popped_crates_list.is_empty() {
        let new_crate_to_push = popped_crates_list.pop_front().unwrap();

        stacks_vector
            .get_mut(dest_stack_index - 1)
            .unwrap()
            .push_back(new_crate_to_push);
    }
}

/**
 * Take an input of the form "move 1 from 2 to 1"
 * and return
 * 1) No. of items to be moved
 * 2) Source Stack
 * 3) Destination Stack
 */
pub fn interpret_move_procedure(move_procedure: &str) -> (usize, usize, usize) {
    let move_count: usize;
    let source_stack: usize;
    let dest_stack: usize;

    // Example Input move 5 from 5 to 9
    // Split on whitespace to get all the parts

    let procedure_parts: Vec<&str> = move_procedure.split_whitespace().collect();

    move_count = procedure_parts.get(1).unwrap().parse::<i32>().unwrap() as usize;
    source_stack = procedure_parts.get(3).unwrap().parse::<i32>().unwrap() as usize;
    dest_stack = procedure_parts.get(5).unwrap().parse::<i32>().unwrap() as usize;

    return (move_count, source_stack, dest_stack);
}
