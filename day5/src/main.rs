use std::{
    collections::LinkedList,
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

use log::{debug, trace};

mod utils;

const INPUT_FILENAME: &str = "input.txt";

fn main() {
    //Set RUST_LOG = "debug" or "trace"
    env_logger::init();

    debug!("Starting Application!");

    // This problem can be interpreted as follows
    // There are N number of stacks, each containing a certain number of elements
    // this is followed by a list of operations where number of elements are transferred from one stack to another

    // We do not need to implement a stack type in Rust, as the LinkedList has push and pop methods already
    // It also has push_front[https://doc.rust-lang.org/std/collections/struct.LinkedList.html#method.push_front],
    // which allows us to insert items at the bottom of the stack,
    // which will make it easier for us to process the puzzle input

    // Note: The example input contains three stacks. But the puzzle input contains nine.
    // To make a program that can solve any problem of this type, we will need to count the number of stacks.

    let stacks_count = utils::get_num_of_stacks(INPUT_FILENAME);

    debug!("No. of stacks: {}", stacks_count);

    // Let's create the stacks, using std-lib LinkedList
    let mut stacks_vector: Vec<LinkedList<char>> = vec![];
    for _ in 0..stacks_count {
        let linked_list_stack: LinkedList<char> = LinkedList::new();
        stacks_vector.push(linked_list_stack);
    }

    debug!(
        "No. of LinkedList stacks pushed into Vector: {}",
        stacks_vector.len()
    );

    utils::insert_crates_into_stacks(INPUT_FILENAME, &mut stacks_vector);

    debug!("Initial state of stacks: {:?}", stacks_vector);

    // We have processed the input and have the data about the initial state of the stacks ! Now time to process the move procedures

    let input_file_lines =
        BufReader::new(File::open(INPUT_FILENAME).expect("Could not open input file!")).lines();

    for input_file_line in input_file_lines {
        if let Ok(current_line) = input_file_line {
            // We are only processing move transactions. Ignore everything else
            if !current_line.contains("move") {
                continue;
            }

            let (move_count, source_stack_index, dest_stack_index) =
                utils::interpret_move_procedure(&current_line);

            trace!(
                "Current Input line: {} | Interpreted: Move {} crates from stack {} to {}",
                current_line,
                move_count,
                source_stack_index,
                dest_stack_index
            );
            // Execute the procedure

            utils::move_crates(
                &mut stacks_vector,
                source_stack_index,
                dest_stack_index,
                move_count,
            );
        } else {
            panic!("Could not read input file lines!");
        }
    }

    debug!("Final state of stacks:\n {:?}", stacks_vector);

    // Prepare the answer stirng by getting the top crate in all stacks
    let mut top_crates_string = String::new();
    for stack in stacks_vector {
        top_crates_string.push(stack.back().unwrap().clone());
    }
    println!("Part 1 | After the rearrangement procedure completes, what crate ends up on top of each stack?\nAnswer: {}",top_crates_string);
}
