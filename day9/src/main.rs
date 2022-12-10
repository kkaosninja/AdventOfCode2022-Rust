use log::{debug, trace, warn};
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};
mod utils;
use utils::MoveDirection;
const INPUT_FILENAME: &str = "puzzle.txt";

fn main() {
    env_logger::init();

    warn!(" -- PART 1 START -- ");

    // Let's try to do this without creating any kind of grid/matrix

    // let starting position be similar to that of an origin on a graph aka 0,0
    let mut part1_rope_data: Vec<[i32; 2]> = Vec::new();
    part1_rope_data.push([0, 0]); // Head data
    part1_rope_data.push([0, 0]); // Tail data

    // This is for Part 1

    // Store all co-ordinates that the tail visited in the form "x,y" in a set
    let mut tail_move_set: HashSet<String> = HashSet::new();
    tail_move_set.insert("0,0".to_string());

    let input_file_lines =
        BufReader::new(File::open(INPUT_FILENAME).expect("Could not open input file")).lines();

    for input_line in input_file_lines {
        if let Ok(current_line) = input_line {
            process_move(current_line, &mut part1_rope_data, &mut tail_move_set);
        } else {
            panic!("Could not read input line!");
        }
    } //for loop

    debug!("Final head position: {:?}", part1_rope_data[0]);
    debug!("Final tail position: {:?}", part1_rope_data[1]);
    trace!(
        "Tail Move Set Count: {}. Moves {:?}",
        tail_move_set.len(),
        tail_move_set
    );

    println!(
        "Part 1 | How many positions does the tail of the rope visit at least once?\nAnswer: {}",
        tail_move_set.len()
    );

    warn!(" -- PART 1 END -- ");
    warn!(" -- PART 2 START -- ");
    // Now solve for part 2

    tail_move_set.clear();

    const END_TAIL_INDEX: usize = 9;

    let mut part2_rope_data: Vec<[i32; 2]> = Vec::new();
    part2_rope_data.push([0, 0]); // Head data
    for _ in 1..=END_TAIL_INDEX {
        part2_rope_data.push([0, 0]);
    }
    tail_move_set.insert("0,0".to_string()); // Add origin aka start to list of moves made by end-tail

    let input_file_lines =
        BufReader::new(File::open(INPUT_FILENAME).expect("Could not open input file")).lines();

    for input_line in input_file_lines {
        if let Ok(current_line) = input_line {
            process_move(current_line, &mut part2_rope_data, &mut tail_move_set);
        } else {
            panic!("Could not read input line!");
        }
    } //for loop

    debug!("Final head position: {:?}", part2_rope_data[0]);
    debug!(
        "Final end-tail position: {:?}",
        part2_rope_data[END_TAIL_INDEX]
    );
    println!(
        "Part 2 | How many positions does the tail of the rope visit at least once?\nAnswer: {}",
        tail_move_set.len()
    );
}

/// The tail_move_set will contain the moves made by the last tail
fn process_move(
    input_line: String,
    rope_data: &mut Vec<[i32; 2]>,
    tail_move_set: &mut HashSet<String>,
) {
    // Get move details
    let (move_type, move_magnitude) =
        get_move_details(input_line).expect("Couild not get move details!");
    trace!("Move Details Detected: {:?} {}", move_type, move_magnitude);

    let mut moves_remaining = move_magnitude;
    while moves_remaining != 0 {
        for current_tail_number in 1..rope_data.len() {
            debug!("--");
            debug!(
                "Processing Head at Index {} and Tail at Index {}",
                current_tail_number - 1,
                current_tail_number
            );
            debug!("Moves Remaining: {}", moves_remaining);

            trace!(
                "Head Index {} Position: {},{} | Tail Index {} Position: {},{} | before head move",
                current_tail_number - 1,
                rope_data[current_tail_number - 1][0],
                rope_data[current_tail_number - 1][1],
                current_tail_number,
                rope_data[current_tail_number][0],
                rope_data[current_tail_number][1]
            );

            // This is where head position gets updated
            // There is a unique scenario that needs to be handled
            // Head gets updated every loop this method gets called, without conditions
            // This works fine for cases where this is only one head and tail
            // But in cases of multiple tails, where each tail is head for the tail that comes after it
            // The "current head" needs to check if it needs to move, because it's also a tail
            // If the "current head" is already touching its own head. Then it does not need to move itself.
            // And if the "current head" does not move, neither do the tails that come after it

            if (current_tail_number > 1)
                && (is_tail_touching_head(rope_data, current_tail_number - 1))
            {
                debug!(
                    "Current head aka Tail at index {} is touching tail at index {}. So current head won't move",
                    current_tail_number - 1,
                    current_tail_number - 2
                );
            
            } else {
                match move_type {
                    MoveDirection::MoveUp => {
                        rope_data.get_mut(current_tail_number - 1).unwrap()[1] += 1;
                    }
                    MoveDirection::MoveDown => {
                        rope_data.get_mut(current_tail_number - 1).unwrap()[1] -= 1;
                    }
                    MoveDirection::MoveLeft => {
                        rope_data.get_mut(current_tail_number - 1).unwrap()[0] -= 1;
                    }
                    MoveDirection::MoveRight => {
                        rope_data.get_mut(current_tail_number - 1).unwrap()[0] += 1;
                    }
                }
            }
            
            trace!(
                "Head Index {} Position: {},{} | Tail Index {} Position: {},{} | before tail move",
                current_tail_number - 1,
                rope_data[current_tail_number - 1][0],
                rope_data[current_tail_number - 1][1],
                current_tail_number,
                rope_data[current_tail_number][0],
                rope_data[current_tail_number][1]
            );
            trace!(
                "Tail Touching Head?: {}",
                is_tail_touching_head(rope_data, current_tail_number)
            );
            // Its easiest to determine tail movement if its done immediately after head is moved
            // Does tail need to move?
            if !is_tail_touching_head(rope_data, current_tail_number) {
                make_tail_touch_head(rope_data, current_tail_number, tail_move_set);
            }

            trace!(
                "Head Index {} Position: {},{} | Tail Index {} Position: {},{} | after tail move",
                current_tail_number - 1,
                rope_data[current_tail_number - 1][0],
                rope_data[current_tail_number - 1][1],
                current_tail_number,
                rope_data[current_tail_number][0],
                rope_data[current_tail_number][1]
            );
        } // for current_tail_number in 1..rope_data.len()
        moves_remaining -= 1;
    } //while loop
}

fn get_move_details(input_line: String) -> Option<(MoveDirection, i32)> {
    let line_parts: Vec<&str> = input_line.split_whitespace().collect();

    let move_type: MoveDirection = match line_parts[0] {
        "U" => MoveDirection::MoveUp,
        "D" => MoveDirection::MoveDown,
        "L" => MoveDirection::MoveLeft,
        "R" => MoveDirection::MoveRight,
        strange_value => {
            panic!("Could not match move expression! Found: {}", strange_value);
        }
    };

    match line_parts[1].parse::<i32>() {
        Ok(move_magnitude) => return Some((move_type, move_magnitude)),
        Err(err) => {
            eprintln!(
                "Failed to parse move magnitude {} due to error {}",
                line_parts[1], err
            );
            return None;
        }
    }
}

fn is_tail_touching_head(rope_data: &Vec<[i32; 2]>, current_tail_number: usize) -> bool {
    let diff_x_abs =
        rope_data[current_tail_number][0].abs_diff(rope_data[current_tail_number - 1][0]);
    let diff_y_abs =
        rope_data[current_tail_number][1].abs_diff(rope_data[current_tail_number - 1][1]);

    if (diff_x_abs <= 1) && (diff_y_abs <= 1) {
        return true;
    }

    return false;
}

/// Function to be called immediately after head is moved. Assuming co-ord distance between is only two in either x or y axis
fn make_tail_touch_head(
    rope_data: &mut Vec<[i32; 2]>,
    tail_number: usize,
    tail_move_set: &mut HashSet<String>,
) {
    //Safety check. Ideally this function is never called if the head and tail are touching
    if is_tail_touching_head(&rope_data, tail_number) {
        return;
    }
    // So distance of two in either x or y values between head and tail

    // Let' handle the most complex case first
    // A Diagonal move is necesary if head and tail are in different x and y values
    if (rope_data[tail_number - 1][0] != rope_data[tail_number][0])
        && (rope_data[tail_number - 1][1] != rope_data[tail_number][1])
    {
        trace!("Diagonal move necessary!");
        //Decide in which direction to move.
        // let diff_x_abs = current_tail_position[0].abs_diff(current_head_position[0]);
        // let diff_y_abs = current_tail_position[1].abs_diff(current_head_position[1]);

        // Decide how to increment/decrement x and y for a diagonal move

        // Is head to the left or right of us? Increment or decrement x
        if rope_data[tail_number - 1][0] > rope_data[tail_number][0] {
            rope_data.get_mut(tail_number).unwrap()[0] += 1;
        } else {
            rope_data.get_mut(tail_number).unwrap()[0] -= 1
        }

        // Is head above or below us? Increment or decrement y
        if rope_data[tail_number - 1][1] > rope_data[tail_number][1] {
            rope_data.get_mut(tail_number).unwrap()[1] += 1;
        } else {
            rope_data.get_mut(tail_number).unwrap()[1] -= 1;
        }
    } else {
        // Not a diagonal move. Only one co-ordinate(x or y) needs to be moved

        // Is head to the left or right of us? Increment or decrement x
        if rope_data[tail_number - 1][0] > rope_data[tail_number][0] {
            rope_data.get_mut(tail_number).unwrap()[0] += 1;
        } else if rope_data[tail_number - 1][0] < rope_data[tail_number][0] {
            rope_data.get_mut(tail_number).unwrap()[0] -= 1

            // Is head above or below us? Increment or decrement y
        } else if rope_data[tail_number - 1][1] > rope_data[tail_number][1] {
            rope_data.get_mut(tail_number).unwrap()[1] += 1;
        } else {
            rope_data.get_mut(tail_number).unwrap()[1] -= 1;
        }
    }
    // tail move finished. Add to moveset
    if tail_number == rope_data.len() - 1 {
        debug!(
            "Adding tail position data to move set for tail number {}",
            tail_number
        );
        let mut new_tail_pos_str = rope_data[tail_number][0].to_string();
        new_tail_pos_str.push(',');
        new_tail_pos_str.push_str(rope_data[tail_number][1].to_string().as_str());

        tail_move_set.insert(new_tail_pos_str);
    }
}
