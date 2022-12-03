use log::{debug, trace};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod game_types;
use game_types::{GameMove, GameResult};

use crate::game_types::PointSystem;

fn main() {
    //Init logger
    //Set RUST_LOG="trace" to enable logging for all log levels.
    // In Powershell, this is `$env:RUST_LOG = "trace"`
    env_logger::init();

    let input_file_handle = File::open("input.txt").expect("Unable to open input file!");
    let input_file_lines = BufReader::new(input_file_handle).lines();

    let mut total_score = 0;

    for input_file_line in input_file_lines {
        if let Ok(current_line) = input_file_line {
            trace!("--");
            trace!("Current line is {}", current_line);

            let coded_moves: Vec<&str> = current_line.split(" ").collect();

            let coded_opponent_move = coded_moves[0];
            let coded_our_move = coded_moves[1];

            debug!(
                "Coded Opponent move {} | Coded Our Move {}",
                coded_opponent_move, coded_our_move
            );

            let our_move: GameMove = interpret_move(coded_our_move);
            let opponent_move: GameMove = interpret_move(coded_opponent_move);

            debug!(
                "Opponent move {:?} | Our Move {:?}",
                opponent_move, our_move
            );

            let game_result = decide_game_result(our_move, opponent_move).unwrap();

            total_score += our_move.get_points();
            total_score += game_result.get_points();

            trace!("Game Result: {:?} | Score Added: {} + {}",game_result,our_move.get_points(),game_result.get_points());
            debug!("Total Score: {}",total_score);
            trace!("--");
        } else {
            panic!("Cannot read the current line! Exiting!");
        }
    }
}

fn interpret_move(coded_move: &str) -> GameMove {
    match coded_move {
        "A" | "X" => GameMove::Rock,
        "B" | "Y" => GameMove::Paper,
        "C" | "Z" => GameMove::Scissors,
        _ => GameMove::Spock,
    }
}

fn decide_game_result(our_move: GameMove, opponent_move: GameMove) -> Option<GameResult> {
    if our_move == opponent_move {
        return Option::Some(GameResult::Draw);
    }

    if our_move == GameMove::Rock {
        if opponent_move == GameMove::Scissors {
            return Option::Some(GameResult::Win);
        }
        return Option::Some(GameResult::Loss);
    }

    if our_move == GameMove::Paper {
        if opponent_move == GameMove::Rock {
            return Option::Some(GameResult::Win);
        }
        return Option::Some(GameResult::Loss);
    }

    if our_move == GameMove::Scissors {
        if opponent_move == GameMove::Paper {
            return Option::Some(GameResult::Win);
        }
        return Option::Some(GameResult::Loss);
    }

    return None;
}
