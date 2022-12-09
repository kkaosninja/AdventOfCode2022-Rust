use log::{debug, trace};
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

    debug!("Starting Application!");

    // Let's try to do this without creating any kind of grid/matrix

    // let starting position be similar to that of an origin on a graph aka 0,0
    let mut head_position = [0, 0];
    let mut tail_position = [0, 0];

    // Store all co-ordinates that the tail visited in the form "x,y" in a set
    let mut tail_move_set: HashSet<String> = HashSet::new();
    tail_move_set.insert("0,0".to_string());

    let input_file_lines =
        BufReader::new(File::open(INPUT_FILENAME).expect("Could not open input file")).lines();

    for input_line in input_file_lines {
        if let Ok(current_line) = input_line {
            process_move(
                current_line,
                &mut head_position,
                &mut tail_position,
                &mut tail_move_set,
            );
        } else {
            panic!("Could not read input line!");
        }
    } //for loop

    debug!("Final head position: {:?}", head_position);
    debug!("Final tail position: {:?}", tail_position);
    trace!(
        "Tail Move Set Count: {}. Moves {:?}",
        tail_move_set.len(),
        tail_move_set
    );

    println!(
        "Part 1 | How many positions does the tail of the rope visit at least once?\nAnswer: {}",
        tail_move_set.len()
    );
}

fn process_move(
    input_line: String,
    current_head_position: &mut [i32; 2],
    current_tail_position: &mut [i32; 2],
    tail_move_set: &mut HashSet<String>,
) {
    // Get move details
    let (move_type, move_magnitude) =
        get_move_details(input_line).expect("Couild not get move details!");

    trace!("Move Details Detected: {:?} {}", move_type, move_magnitude);
    trace!(
        "Head Position: {},{} | Tail Position: {},{}",
        current_head_position[0],
        current_head_position[1],
        current_tail_position[0],
        current_tail_position[1]
    );
    let mut moves_remaining = move_magnitude;
    while moves_remaining != 0 {
        match move_type {
            MoveDirection::MoveUp => {
                current_head_position[1] += 1;
            }
            MoveDirection::MoveDown => {
                current_head_position[1] -= 1;
            }
            MoveDirection::MoveLeft => {
                current_head_position[0] -= 1;
            }
            MoveDirection::MoveRight => {
                current_head_position[0] += 1;
            }
        }
        trace!(
            "Head Position: {},{} | Tail Position before tail move: {},{}",
            current_head_position[0],
            current_head_position[1],
            current_tail_position[0],
            current_tail_position[1]
        );
        trace!(
            "Tail Touching Head?: {}",
            is_tail_touching_head(current_head_position, current_tail_position)
        );
        // Its easiest to determine tail movement if its done immediately after head is moved
        // Does tail need to move?
        if !is_tail_touching_head(current_head_position, current_tail_position) {
            make_tail_touch_head(current_head_position, current_tail_position, tail_move_set);
        }

        moves_remaining -= 1;
        trace!(
            "Head Position: {},{} | Tail Position after tail move: {},{}",
            current_head_position[0],
            current_head_position[1],
            current_tail_position[0],
            current_tail_position[1]
        );
    }
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

fn is_tail_touching_head(
    current_head_position: &[i32; 2],
    current_tail_position: &[i32; 2],
) -> bool {
    let diff_x_abs = current_tail_position[0].abs_diff(current_head_position[0]);
    let diff_y_abs = current_tail_position[1].abs_diff(current_head_position[1]);

    if (diff_x_abs <= 1) & (diff_y_abs <= 1) {
        return true;
    }

    return false;
}

/// Function to be called immediately after head is moved. Assuming co-ord distance between is only two in either x or y axis
fn make_tail_touch_head(
    current_head_position: &[i32; 2],
    current_tail_position: &mut [i32; 2],
    tail_move_set: &mut HashSet<String>,
) {
    //Safety check. Ideally this function is never called if the head and tail are touching
    if is_tail_touching_head(current_head_position, current_tail_position) {
        return;
    }
    // So distance of two in either x or y values between head and tail

    // Let' handle the most complex case first
    // A Diagonal move is necesary if head and tail are in different x and y values
    if (current_head_position[0] != current_tail_position[0])
        & (current_head_position[1] != current_tail_position[1])
    {
        trace!("Diagonal move necessary!");
        //Decide in which direction to move.
        // let diff_x_abs = current_tail_position[0].abs_diff(current_head_position[0]);
        // let diff_y_abs = current_tail_position[1].abs_diff(current_head_position[1]);

        // Decide how to increment/decrement x and y for a diagonal move

        // Is head to the left or right of us? Increment or decrement x
        if current_head_position[0] > current_tail_position[0] {
            current_tail_position[0] += 1;
        } else {
            current_tail_position[0] -= 1
        }

        // Is head above or below us? Increment or decrement y
        if current_head_position[1] > current_tail_position[1] {
            current_tail_position[1] += 1;
        } else {
            current_tail_position[1] -= 1;
        }
    } else {
        // Not a diagonal move. Only one co-ordinate(x or y) needs to be moved

        // Is head to the left or right of us? Increment or decrement x
        if current_head_position[0] > current_tail_position[0] {
            current_tail_position[0] += 1;
        } else if current_head_position[0] < current_tail_position[0] {
            current_tail_position[0] -= 1

            // Is head above or below us? Increment or decrement y
        } else if current_head_position[1] > current_tail_position[1] {
            current_tail_position[1] += 1;
        } else {
            current_tail_position[1] -= 1;
        }
    }
    // tail move finished. Add to moveset
    let mut new_tail_pos_str = current_tail_position[0].to_string();
    new_tail_pos_str.push(',');
    new_tail_pos_str.push_str(current_tail_position[1].to_string().as_str());

    tail_move_set.insert(new_tail_pos_str);
}
