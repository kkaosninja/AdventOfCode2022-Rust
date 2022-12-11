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

    debug!("Initial CPU State {:?}", cpu_state);
    for input_file_line in input_file_lines {
        if let Ok(current_line) = input_file_line {
            debug!("--");
            debug!("Main | Current CPU State before executor {:?}", cpu_state);
            executor(&current_line, &mut cpu_state, &mut int_signal_strengths);
            debug!("Main | Current CPU State after executor {:?}", cpu_state);
        }
    }

    println!(
        "Part 1 | What is the sum of these interesting signal strengths?\nAnswer: {}",
        int_signal_strengths.iter().sum::<i32>()
    );
}

fn executor(input_line: &String, cpu_state: &mut CpuState, int_signal_strengths: &mut Vec<i32>) {
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