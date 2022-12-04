use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use log::{debug, trace};

fn main() {
    //Set env var RUST_LOG = "trace" or "debug"
    env_logger::init();

    let input_file_reader =
        BufReader::new(File::open("input.txt").expect("Could not open input file!"));

    let mut contained_pairs_count = 0;
    let mut overlap_pairs_count = 0;

    for input_file_line in input_file_reader.lines() {
        if let Ok(current_line) = input_file_line {
            let line_parts: Vec<&str> = current_line.split(',').collect::<Vec<&str>>();

            debug!(
                "First Elf range str: {} | Second elf range str: {}",
                line_parts[0], line_parts[1]
            );

            let (elf1_range_start, elf1_range_end) = get_assignment_range(line_parts[0]);
            trace!("Elf 1 Range: {} - {}", elf1_range_start, elf1_range_end);

            let (elf2_range_start, elf2_range_end) = get_assignment_range(line_parts[1]);
            trace!("Elf 2 Range: {} - {}", elf2_range_start, elf2_range_end);

            // Check if one range contains the other
            // If true increment count and go on to next line
            // NOTE: If one pair contains the other, it automatically implies overlap
            // So we will not specifically check overlap for those pairs
            if elf1_range_start <= elf2_range_start && elf1_range_end >= elf2_range_end {
                debug!("Elf 1 range contains Elf 2 range!");
                contained_pairs_count += 1;
                overlap_pairs_count += 1;
                continue;
            }

            if elf2_range_start <= elf1_range_start && elf2_range_end >= elf1_range_end {
                debug!("Elf 2 range contains Elf 1 range!");
                contained_pairs_count += 1;
                overlap_pairs_count += 1;
                continue;
            }

            // Check overlap for pairs where one does not contain the other
            // The problem can be reduced to one where we find one or more common elements in two sequences

            let mut elf1_range_set: HashSet<i32> = HashSet::new();
            for i in elf1_range_start..=elf1_range_end {
                elf1_range_set.insert(i);
            }

            let mut elf2_range_set: HashSet<i32> = HashSet::new();
            for i in elf2_range_start..=elf2_range_end {
                elf2_range_set.insert(i);
            }

            let set_intersection_count = elf1_range_set.intersection(&elf2_range_set).count();

            if set_intersection_count > 0 {
                debug!(
                    "Elf 1 range and Elf 2 range overlap! Size of overlap: {}",
                    set_intersection_count
                );
                overlap_pairs_count += 1;
            }
        } else {
            panic!("Could not read input line. Exiting!");
        }
    }

    println!("No. of contained pairs: {}", contained_pairs_count);
    println!("No. of overlapping pairs: {}", overlap_pairs_count);
}

fn get_assignment_range(range_str: &str) -> (i32, i32) {
    let range_parts = range_str.split('-').collect::<Vec<&str>>();

    let range_start = range_parts[0].parse::<i32>().unwrap();
    let range_end = range_parts[1].parse::<i32>().unwrap();

    return (range_start, range_end);
}
