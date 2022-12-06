use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

use log::{debug, trace};

const INPUT_FILENAME: &str = "example.txt";

fn main() {
    env_logger::init();

    debug!("Starting Application!");

    let input_string = read_to_string(INPUT_FILENAME).expect("Unable to read input file!");

    // To solve this problem, we need a moving window of size 4 or 14. A queue data structure would be useful
    // Let's use a VecDeque for this from std::Collections
    // push_back() to enqueue. pop_front() to dequeue

    trace!("Input String => {}", input_string);

    debug!("Starting Part 1 Solution code now");

    //NOTE: Due to similarity in the solution algorithm, we can use the same function to
    // get the answers for both parts 1 and 2. The only variable being the size of the "window"

    // Window size represents the no. of distinct characters present in a contiguous sequence which denote the start of an appropriate marker
    // For part 1, the marker is start-of-packet. For part 2, it's start-of-message

    // For Part 1, pass window size of 4 to the method .
    let start_of_packet_marker = get_marker_index::<4>(&input_string);

    println!(
        "Part 1 | How many characters need to be processed before the first start-of-packet marker is detected?\nAnswer: {}",
        start_of_packet_marker
    );

    debug!("Starting Part 2 Solution code now");

    // For part 2, pass window size of 14 to the method
    let start_of_message_marker = get_marker_index::<14>(&input_string);

    println!(
        "Part 2 | How many characters need to be processed before the first start-of-message marker is detected?\nAnswer: {}",
        start_of_message_marker
    );
}

fn get_marker_index<const WINDOW_SIZE: i32>(input_string: &String) -> i32 {
    // This is the value we will return.
    let mut marker_index: i32 = 0;

    // This is the queue we will be using that will represent the moving window
    let mut packet_window: VecDeque<char> = VecDeque::with_capacity((WINDOW_SIZE + 1) as usize);

    // Time to process the input!

    let mut input_char_indices = input_string.char_indices();
    while let Some((char_index, packet_char)) = input_char_indices.next() {
        debug!("Current Packet Window Contents: {:?}", packet_window);
        trace!("Current char: {}", packet_char);

        //Move the window forward
        //Also handle scenario where window is not yet full

        //Add new character to window
        trace!("Adding char {} to the window", packet_char);
        packet_window.push_back(packet_char);

        // Only dequeue if packet window size is greater than window_size
        if (packet_window.len() as i32) > WINDOW_SIZE {
            trace!(
                "Queue: {:?} | Queue Size > {}. De-queueing now.",
                packet_window,
                WINDOW_SIZE
            );
            let dequeued_char = packet_window.pop_front().unwrap();
            trace!("De-queued char {} from the queue", dequeued_char);
        }

        // Packet window still isn't full. So we cannot check for distinct characters yet.
        // Therefore go to next iteration
        if (char_index as i32) < WINDOW_SIZE {
            continue;
        }

        //  Check if all characters in the window are unique
        if all_chars_are_unique(&packet_window) {
            // We found the start of the marker index
            // Set the value and break the loop
            marker_index = (char_index + 1) as i32;
            break;
        }
    } //while loop

    return marker_index;
}

fn all_chars_are_unique(packet_window: &VecDeque<char>) -> bool {
    //Get size of window

    //Add all chars to a Hashset
    let mut char_set: HashSet<char> = HashSet::new();

    for window_char in packet_window {
        char_set.insert(window_char.clone());
    }

    if char_set.len() == packet_window.len() {
        return true;
    }

    return false;
}
