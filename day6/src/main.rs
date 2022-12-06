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

    // To solve this problem, we need a moving window of size 4. A queue data structure would be useful
    // Let's use a VecDeque for this from std::Collections
    // push_back() to enqueue. pop_front() to dequeue

    let mut packet_window: VecDeque<char> = VecDeque::with_capacity(4);

    //Set marker to minimum possible value. We will update
    let mut start_of_packet_marker: i32 = 0;

    // The Window is full
    // Let's go through the string

    trace!("Input String => {}", input_string);
    let mut input_char_indices = input_string.char_indices();

    while let Some((char_index, packet_char)) = input_char_indices.next() {
        debug!("Current Packet Window Contents: {:?}", packet_window);
        trace!("Current char: {}", packet_char);

        //Move the window forward
        //Also handle scenario where window is not yet full

        //Add new character to window
        trace!("Adding char {} to the window", packet_char);
        packet_window.push_back(packet_char);

        // Only dequeue if packet window size is greater than 4
        if packet_window.len() > 4 {
            trace!(
                "Queue: {:?} | Queue Size > 4. De-queueing now.",
                packet_window
            );
            let dequeued_char = packet_window.pop_front().unwrap();
            trace!("De-queued char {} from the queue", dequeued_char);
        }

        // Skip until the 4th character
        if char_index < 4 {
            continue;
        }

        //  Check if all characters in the window are unique
        if all_chars_are_unique(&packet_window) {
            // We found the start of the start-of-packet marker
            // Set the value and break the loop

            start_of_packet_marker = (char_index + 1) as i32;
            break;
        }
    }

    println!("Part 1 | Start of packet marker: {}", start_of_packet_marker);
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
