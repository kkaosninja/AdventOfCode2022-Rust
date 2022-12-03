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

    let mut total_score_part1 = 0;
    let mut total_score_part2 = 0;

    for input_file_line in input_file_lines {
        if let Ok(current_line) = input_file_line {
            debug!("--");
            trace!("Current line is {}", current_line);

            let coded_moves: Vec<&str> = current_line.split(" ").collect();

            let coded_opponent_move = coded_moves[0];
            let coded_our_move = coded_moves[1];

            debug!(
                "Coded Our move {} | Coded Opponent Move {}",
                coded_our_move, coded_opponent_move
            );

            let our_move: GameMove = interpret_move(coded_our_move);
            let opponent_move: GameMove = interpret_move(coded_opponent_move);

            debug!(
                "Our move {:?} | Opponent Move {:?}",
                our_move, opponent_move
            );

            let game_result = decide_game_result(our_move, opponent_move);

            total_score_part1 += our_move.get_points();
            total_score_part1 += game_result.get_points();

            debug!("Game Result: {:?}", game_result);
            trace!(
                "Game Result: {:?} | Score Added: {} + {}",
                game_result,
                our_move.get_points(),
                game_result.get_points()
            );
            debug!("Total Score: {}", total_score_part1);

            // -- Below code strictly for Part 2
            let desired_result: GameResult = interpret_coded_desired_result(coded_our_move);
            let ideal_move = get_move_for_desired_result(opponent_move, desired_result);
            total_score_part2 += ideal_move.get_points();
            total_score_part2 += desired_result.get_points();

            trace!("--");
        } else {
            panic!("Cannot read the current line! Exiting!");
        }
    }

    println!("Part 1 Solution aka Total Score: {}", total_score_part1);
    println!("Part 2 Solution aka Total Score: {}", total_score_part2);

}

fn interpret_move(coded_move: &str) -> GameMove {
    match coded_move {
        "A" | "X" => GameMove::Rock,
        "B" | "Y" => GameMove::Paper,
        "C" | "Z" => GameMove::Scissors,
        _ => GameMove::Spock,
    }
}

fn decide_game_result(our_move: GameMove, opponent_move: GameMove) -> GameResult {
    if our_move == opponent_move {
        return GameResult::Draw;
    }

    if our_move == GameMove::Rock && opponent_move == GameMove::Scissors {
        return GameResult::Win;
    }

    if our_move == GameMove::Paper && opponent_move == GameMove::Rock {
        return GameResult::Win;
    }

    if our_move == GameMove::Scissors && opponent_move == GameMove::Paper {
        return GameResult::Win;
    }

    GameResult::Loss
}

// Below functions for part 2 solution

fn interpret_coded_desired_result(coded_desired_result: &str) -> GameResult {
    match coded_desired_result {
        "Y" => return GameResult::Draw,
        "X" => return GameResult::Loss,
        "Z" => return GameResult::Win,
        _ => return GameResult::Wtf,
    }
}

/// Get the move we need to make in order to achieve the desired result, given opponent move
fn get_move_for_desired_result(opponent_move: GameMove, desired_result: GameResult) -> GameMove {
    if desired_result == GameResult::Draw {
        return opponent_move;
    }

    if desired_result == GameResult::Win {
        match opponent_move {
            GameMove::Rock => return GameMove::Paper,
            GameMove::Paper => return GameMove::Scissors,
            GameMove::Scissors => return GameMove::Rock,
            _ => return GameMove::Spock,
        }
    }

    if desired_result == GameResult::Loss {
        match opponent_move {
            GameMove::Rock => return GameMove::Scissors,
            GameMove::Paper => return GameMove::Rock,
            GameMove::Scissors => return GameMove::Paper,
            _ => return GameMove::Spock,
        }
    }

    return GameMove::Spock;
}
