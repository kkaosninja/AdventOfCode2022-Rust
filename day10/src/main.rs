use log::{debug, trace, warn};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod utils;
use utils::{get_instruction_type, CpuState, InstructionType};

const INPUT_FILENAME: &str = "puzzle.txt";

fn main() {
    env_logger::init();
    debug!("Starting Application!");

    let input_file_lines =
        BufReader::new(File::open(INPUT_FILENAME).expect("Could not open input file")).lines();

    let mut cpu_state = utils::CpuState {
        register_x: 1,
        cycle_count: 0,
    };

    let mut int_signal_strengths: Vec<i32> = Vec::new();

    // For Part 2 answer
    let mut crt_display_row: String = String::new();

    debug!("Initial CPU State {:?}", cpu_state);
    for input_file_line in input_file_lines {
        if let Ok(current_line) = input_file_line {
            debug!("--");
            debug!("Main | Current CPU State before executor {:?}", cpu_state);
            executor(
                &current_line,
                &mut cpu_state,
                &mut int_signal_strengths,
                &mut crt_display_row,
            );
            debug!("Main | Current CPU State after executor {:?}", cpu_state);
        }
    }

    println!(
        "Part 1 | What is the sum of these interesting signal strengths?\nAnswer: {}",
        int_signal_strengths.iter().sum::<i32>()
    );

    println!("Part 2 | Pattern:\n{}", crt_display_row);
}

fn executor(
    input_line: &String,
    cpu_state: &mut CpuState,
    int_signal_strengths: &mut Vec<i32>,
    crt_display_row: &mut String,
) {
    trace!("Executing line: {}", input_line);

    let mut cycles_remaining = 0;

    let instruction_type = get_instruction_type(input_line);

    match instruction_type {
        InstructionType::Noop => cycles_remaining = 1,
        InstructionType::Addx => cycles_remaining = 2,
    }

    while cycles_remaining > 0 {
        // Sometimes the moment when we need to get the signal strength is in the middle
        // of an execution cycle. This loop exists for that reason
        trace!("In Loop | Current CPU State {:?}", cpu_state);

        trace!(
            "Cycles remaining: {} | Incrementing cycle count",
            cycles_remaining
        );
        cpu_state.cycle_count += 1;

        // Code for part 2

        warn!(
            "CRT Drawing pixel at position {}. Sprite middle position at {} or {}",
            cpu_state.cycle_count - 1,
            cpu_state.register_x,
            cpu_state.register_x + 40*((cpu_state.cycle_count as i32 - 1)/40)
        );
        if is_crt_drawing_sprite(cpu_state) {
            warn!("Adding Lit Pixel for CPU Cycle {}", cpu_state.cycle_count);
            crt_display_row.push('#');
        } else {
            warn!("Adding Dark Pixel for CPU Cycle {}", cpu_state.cycle_count);
            crt_display_row.push('.');
        }
        //Add newline after every 40th character
        if (cpu_state.cycle_count >= 40) && (cpu_state.cycle_count % 40 == 0) {
            crt_display_row.push('\n');
        }
        debug!("CRT Draw String:\n{}", crt_display_row);

        /*
        Technically, the sequence for cycle counts at which need to record signal strebgths
        given in the Puzzle is an Arithmetic Progression
        20 + 40*i. 20 is the first element. 40 is the difference

        Therefore, to find out if our cycle count is interesting aka part of the AP sequence,
        we need to find out
        if (signal_strength - 20) % 40 == 0

        */
        let mut cycle_count_interesting = false;
        if (cpu_state.cycle_count >= 60) && ((cpu_state.cycle_count as i32 - 20) % 40) == 0 {
            cycle_count_interesting = true;
        } else if cpu_state.cycle_count == 20 {
            cycle_count_interesting = true;
        }
        // Record signal strength
        if cycle_count_interesting {
            let int_signal_strength = cpu_state.get_signal_strength();
            warn!(
                "Cycle Count: {}. Recording Interesting Signal Strength {}",
                cpu_state.cycle_count, int_signal_strength
            );
            int_signal_strengths.push(int_signal_strength);
        }

        match instruction_type {
            InstructionType::Noop => {
                // We don't need more than one iteration of the loop
                break;
            }
            InstructionType::Addx => {
                // Execute this instruction in last cycle
                if cycles_remaining == 1 {
                    let operand_value = input_line
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .get(1)
                        .expect("Could not index new register value")
                        .parse::<i32>()
                        .expect("Could not parse new register value");
                    cpu_state.register_x += operand_value;
                }
            }
        } //match case

        cycles_remaining -= 1;
    } //while loop
}

fn is_crt_drawing_sprite(cpu_state: &CpuState) -> bool {
    // CRT always draws in pixel (current_cycle-1)
    let current_draw_position: i32 = (cpu_state.cycle_count as i32) - 1;

    // Since x value is negative
    if cpu_state.register_x < 0 {
        return false;
    }

        /*
    On cycles 41 and above. CRT draws on rows other than first.
    And register X only determines the "horizontal" position. 
    Meaning it will never go above 40
    If the register_X has a value of 35, but the CPU is on cycle 100
    the CRT will be drawing on the third row of the 40x6 grid.
    So the "effective" sprite position will be have to be determined
    by adding a multiple of 40 to it.
    */

    let mut sprite_middle_position = cpu_state.register_x;

    if current_draw_position > 40 {
        sprite_middle_position += 40*(current_draw_position/40);
    }

    //Check if CRT is drawing one of the sprite pixels
    if ((sprite_middle_position - 1) == current_draw_position)
        || (sprite_middle_position == current_draw_position)
        || ((sprite_middle_position + 1) == current_draw_position)
    {
        return true;
    }

    return false;
}
