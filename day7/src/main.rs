use log::{debug, trace, warn};
use std::{cell::RefCell, fs::read_to_string, rc::Rc, str::FromStr};

mod utils;
use utils::{get_line_type, InputLineType, PuzzleDir, PuzzleFile};

const INPUT_FILENAME: &str = "puzzle.txt";

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
        and then call process_input() recursively again. In addition to the subdir reference, we also pass the next line number
        to process when making the recursive call

        Once it returns(as a result of it encountering a "cd .." line), we will use the returned line number
        and continue processing the input lines
    */

    /*
       Since we are using a recursive data structure, and a recursive function to fill it with data,
       we will run into problems with borrow checker. At any point of time, there will be multiple references
       to a dir data structure. Sometimes that reference holder may need to make changes to the dir(adding files and subdirs).

       For this reason, a simple implementation will run into problems with Rust's borrow-checker
       Remember rust ownership rules https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-rules
       1) Every value can only have one owner at any given time
       2) Value dropped as soon as it goes out of scope

       We cannot pass around multiple mutable references to a single dir data structure, without getting compile errors.
        At any given time, for safe programming, there should always be a single holder.
       Implemented by PuzzleDir
       For this reason we will need a container type who will hold this value for us.
       1) std::rc::Rc - Reference Counted smart pointer - https://doc.rust-lang.org/book/ch15-04-rc.html

       We can always use clone() on an Rc container, thereby ensuring value is not dropped even after a temporary "move" outside scope.

       2) The Rc itself will contain an instance of RefCell, which will in turn hold an instance of PuzzleDir
           https://doc.rust-lang.org/book/ch15-05-interior-mutability.html

       By calling borrow() and borrow_mut() on RefCell, we can get immutable and mutable(only when necessary) references to the same instances of PuzzleDir

       The checking for ownership rules for this happens at run-time. So its absolutely possible to cause a panic if we misuse this.

       By combining an Rc and RefCell, we can create a tree data structure that is safe(at least at compile time) to work with.
    */

    // First line of input is always "cd /". We start at the root dir
    let root_dir_ref = Rc::new(RefCell::new(PuzzleDir::new("/")));

    // Start at second line aka 1
    // Use clone() to increment pointer count. Otherwise we cannot use it later as we "moved" the only available pointer
    debug!("Being Processing Input!");
    let final_line_number = process_input(&input_file_lines, 1, root_dir_ref.clone());
    debug!("Line number returned: {}", final_line_number);

    let root_dir_size = root_dir_ref.clone().borrow().get_size();
    debug!("Total size of root dir {}", root_dir_size);

    debug!("Now adding all dir sizes to a Vector!");

    // Vector containing all dir sizes
    let mut dir_size_vec: Vec<usize> = Vec::new();

    get_dir_sizes(&mut dir_size_vec, root_dir_ref.clone());
    trace!("{:?}", dir_size_vec);

    // Solve for Part 1
    // Double de-structuring in filter() - https://doc.rust-lang.org/core/iter/trait.Iterator.html#examples-14
    let part1_answer: usize = dir_size_vec
        .iter()
        .filter(|&&x| x <= 100_000 as usize)
        .sum();

    // Solve for Part 2
    let free_space_available = 70_000_000 - root_dir_size;
    debug!("Current free space available is {}", free_space_available);

    // We need 30_000_000 for the update. How much more do we need to free?
    let space_to_be_freed = 30_000_000 - free_space_available;

    // Find size of smallest directory to be deleted
    let mut part2_answer: usize = 0;
    dir_size_vec
        .iter()
        .filter(|&&x| x > space_to_be_freed)
        .for_each(|&dir_size| {
            if (part2_answer == 0) | (dir_size < part2_answer) {
                part2_answer = dir_size;
            }
        });

    println!(
        "Part 1 | What is the sum of the total sizes of those directories with a total size of at most 100000?\nAnswer: {}",
        part1_answer
    );
    println!(
        "Part 2 | What is the total size of the smallest directory to be deleted to create 30_000_000 of free space?\nAnswer: {}",
        part2_answer
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

        trace!("We are in directory: {}", this_dir.borrow().name);
        trace!(
            "Current line index: {} | Line: {}",
            current_line_index,
            current_line
        );

        match get_line_type(current_line) {
            // Is this the "ls" line
            InputLineType::CommandLsDir => {
                trace!("Detected ls command. Proceeding to next line!");
                current_line_index += 1;
                continue;
            }

            // Is this a "cd .." line?
            InputLineType::CommandCdIntoParentDir => {
                trace!(
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
                trace!(
                    "Adding new subdir {} to dir {}",
                    new_subdir_ref.borrow().name,
                    this_dir.borrow().name
                );
                this_dir.borrow_mut().sub_dirs.push(new_subdir_ref);

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

                trace!(
                    "Adding new file {} to dir {}",
                    new_file_name,
                    this_dir.borrow().name
                );

                this_dir.borrow_mut().files.push(new_file);
                // Apparently this is how the above line works
                // Confused as std::rc::Rc does not have a borrow_mut() implemented
                // {
                //     let ref this = this_dir;
                //     this.try_borrow_mut().expect("already borrowed")
                // }.files.push(new_file);

                current_line_index += 1;
                continue;
            }

            // Is this a "cd subdir_name" line?
            // Note: this will only occur after the ls output is done
            // so no need to worry about a dir not existing
            InputLineType::CommandCdIntoDir => {
                // Get the name of the directory
                let subdir_name = String::from_str(
                    *current_line_parts
                        .get(2)
                        .expect("Could not get subdir name"),
                )
                .expect("Could not convert subdir name from &str to String");

                trace!("Subdir to change into is {}", subdir_name);

                // Search for the subdir and find the reference
                let mut next_subdir_index: usize = 0;

                for i in 0..this_dir.borrow().sub_dirs.len() {
                    let search_subdir_name = this_dir.borrow().sub_dirs[i].borrow().name.clone();
                    // trace!(
                    //     "Searching. Current index subdir name is {} at index {}",
                    //     search_subdir_name,
                    //     i
                    // );
                    if search_subdir_name == subdir_name {
                        trace!("Subdir search successful. Subdir found. Index is {}", i);
                        next_subdir_index = i;
                        break;
                    }
                }

                if next_subdir_index == 0 {
                    warn!("Subdir search may have failed!");
                }

                let next_subdir_ref = this_dir
                    .borrow()
                    .sub_dirs
                    .get(next_subdir_index)
                    .expect("Could not fetch dir with index")
                    .clone();

                //Recursive call. Pass reference to subdirectory and next line number to process
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

fn get_dir_sizes(dir_size_vec: &mut Vec<usize>, current_dir_ref: Rc<RefCell<PuzzleDir>>) {
    // Add details of current directory

    let current_dir_name = current_dir_ref.borrow().name.clone();
    let current_dir_size = current_dir_ref.borrow().get_size();
    trace!(
        "Inserting data for dir {} of size {}",
        current_dir_name,
        current_dir_size
    );

    dir_size_vec.push(current_dir_size);

    // Condition to terminate recursion
    // If this is a leaf node, aka a directory that has no subdirs, then we return
    if (current_dir_size == 0) | (current_dir_ref.borrow().sub_dirs.len() == 0) {
        return;
    }

    for i in 0..current_dir_ref.borrow().sub_dirs.len() {
        let sub_dir_ref = current_dir_ref
            .borrow()
            .sub_dirs
            .get(i)
            .expect("Could not fetch subdir")
            .clone();
        debug!(
            "Calling get_dir_sizes on subdir {} of dir {}",
            sub_dir_ref.borrow().name,
            current_dir_name
        );
        get_dir_sizes(dir_size_vec, sub_dir_ref);
    }
}
