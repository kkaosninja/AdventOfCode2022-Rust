pub trait PointSystem {
    /// Return the number of points awarded to the player for this move/result
    /// Refer: https://adventofcode.com/2022/day/2
    fn get_points(&self) -> i32;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameResult {
    Win,
    Loss,
    Draw,
}

impl PointSystem for GameResult {
    fn get_points(&self) -> i32 {
        match self {
            Self::Win => return 6,
            Self::Loss => return 0,
            Self::Draw => return 3,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameMove {
    Rock,
    Paper,
    Scissors,
}

impl PointSystem for GameMove {
    fn get_points(&self) -> i32 {
        match self {
            Self::Rock => return 1,
            Self::Paper => return 2,
            Self::Scissors => return 3,
        }
    }
}
