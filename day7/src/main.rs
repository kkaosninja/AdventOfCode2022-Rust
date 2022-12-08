use log::{debug, trace};
use std::{cell::RefCell, fs::read_to_string, rc::Rc};

mod utils;
use utils::{get_line_type, InputLineType, PuzzleDir, PuzzleFile};

const INPUT_FILENAME: &str = "example.txt";

fn main() {
    env_logger::init();

    debug!("Starting application");

    let input_file_lines: Vec<String> = read_to_string(INPUT_FILENAME)
        .expect("Could not read input file!")
        .split('\n')
        .map(|line| String::from(line))
        .collect();

    debug!("Read in {} lines from input file", input_file_lines.len());

    /*
    To ensure we have a full representation of the directory tree
    we will use a recursive function to sollect the data
    It should be a function that can be called to collect data for any directory at any level
    We will start by calling the function on the root dir, as per the first line in the input => "cd /"

    This function will be process_input().
    */

    // The data structures to represent the files and dirs are prsent in the utils module(utils.rs)

    /*
    There is a pattern is that is obvious to see in both the examples and puzzle input.
    Every time we encounter a "cd dir_name", it is followed by an ls dir_name
    which in turn is followed by the contents of that directory.

    Once we are done processing the output of the ls command for the current directory.

    There will be two possibilities. Either a "cd .." or "cd subdir_name"
    1) In case of a "cd ..", we will simply return. The return value will be the next line number,
    which the calling function will consume, and decide what to do next

    2) In case of a "cd subdir_name", we get a reference to this subdir's instance from our current dir's data structure
        and then call process_input() recursively again.

    In case of (2) we make a
    */

    // First line of input is always "cd /". We start at the root dir
    let root_dir_ref = Rc::new(RefCell::new(PuzzleDir::new("/")));

    // Start at second line aka 1
    // Use clone() to increment pointer count. Otherwise we cannot use it later as moved the only available pointer
    let final_line_number = process_input(&input_file_lines, 1, root_dir_ref.clone());
    debug!("Line number returned: {}", final_line_number);

    debug!(
        "Total size of root dir {}",
        root_dir_ref.clone().borrow().get_size()
    );
}

// Returned value is the next value to be processed
fn process_input(
    input_file_lines: &Vec<String>,
    input_line_index: usize,
    this_dir: Rc<RefCell<PuzzleDir>>,
) -> usize {
    // Process the ls command entries
    // Check for safety anyway

    // Process ls output for this directory
    let mut current_line_index = input_line_index;

    while current_line_index < input_file_lines.len() {
        let current_line = input_file_lines
            .get(current_line_index)
            .expect("Could not fetch input line string from vector");
        let current_line_parts: Vec<&str> = current_line.split_whitespace().collect();

        debug!("We are in directory: {}", this_dir.clone().borrow().name);
        debug!(
            "Current line index: {} | Line: {}",
            current_line_index, current_line
        );

        match get_line_type(current_line) {
            // Is this the "ls" line
            InputLineType::CommandLsDir => {
                debug!("Detected ls command. Proceeding to next line!");
                current_line_index += 1;
                continue;
            }

            // Is this a "cd .." line?
            InputLineType::CommandCdIntoParentDir => {
                debug!(
                    "Detected a cd .. line. Returning line number {}",
                    current_line_index + 1
                );
                return current_line_index + 1;
            }

            // Is this subdir? aka "dir abcd"
            InputLineType::LsOutputDir => {
                let new_subdir_ref = Rc::new(RefCell::new(PuzzleDir::new(
                    current_line_parts
                        .get(1)
                        .expect("Could not get subdir name"),
                )));
                debug!(
                    "Adding new subdir {} to dir {}",
                    new_subdir_ref.borrow().name,
                    this_dir.borrow().name
                );
                this_dir.clone().borrow_mut().sub_dirs.push(new_subdir_ref);

                current_line_index += 1;
                continue;
            }

            // Is this an entry for a file? aka "123456 file.txt"
            InputLineType::LsOutputFile => {
                let new_file_name = *current_line_parts
                    .get(1)
                    .expect("Could not get the file name");
                let new_file_size = current_line_parts
                    .get(0)
                    .expect("Could not get the file size")
                    .parse::<usize>()
                    .expect("Could not parse the file size");

                let new_file = PuzzleFile::new(new_file_name, new_file_size);

                debug!(
                    "Adding new file {} to dir {}",
                    new_file_name,
                    this_dir.borrow().name
                );

                this_dir.clone().borrow_mut().files.push(new_file);

                current_line_index += 1;
                continue;
            }

            // Is this a "cd subdir_name" line?
            // Note: this will only occur after the ls output is done
            // so no need to worry about a dir not existing
            InputLineType::CommandCdIntoDir => {
                // Get the name of the directory
                let subdir_name = *current_line_parts
                    .get(1)
                    .expect("Could not get subdir name");

                // Search for the subdir and find the reference
                let mut next_subdir_index: usize = 0;

                for i in 0..this_dir.clone().borrow().sub_dirs.len() {
                    if this_dir.clone().borrow().sub_dirs[i].clone().borrow().name == subdir_name {
                        next_subdir_index = i;
                        break;
                    }
                }

                let next_subdir_ref = this_dir
                    .clone()
                    .borrow()
                    .sub_dirs
                    .get(next_subdir_index)
                    .expect("Could not fetch dir with index")
                    .clone();

                //FIXME: We need to have a recursive call to finish out the program
                debug!(
                    "Moving into subdir {} of dir {} by making recursive call",
                    next_subdir_ref.borrow().name,
                    this_dir.borrow().name
                );
                current_line_index =
                    process_input(input_file_lines, current_line_index + 1, next_subdir_ref);
            }
        } // match
    } // while loop

    return current_line_index;
}

