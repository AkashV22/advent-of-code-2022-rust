use std::{
    fs::File,
    io,
    io::{BufRead, BufReader},
    path::PathBuf,
};

enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score_against(&self, opponent: &Move) -> u32 {
        let base_score: u32 = match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        };

        let result: GameResult = match (self, opponent) {
            (Self::Rock, Self::Rock) => GameResult::Draw,
            (Self::Rock, Self::Paper) => GameResult::Lose,
            (Self::Rock, Self::Scissors) => GameResult::Win,
            (Self::Paper, Self::Rock) => GameResult::Win,
            (Self::Paper, Self::Paper) => GameResult::Draw,
            (Self::Paper, Self::Scissors) => GameResult::Lose,
            (Self::Scissors, Self::Rock) => GameResult::Lose,
            (Self::Scissors, Self::Paper) => GameResult::Win,
            (Self::Scissors, Self::Scissors) => GameResult::Draw,
        };

        base_score + result.score()
    }
}

fn player_key_to_move(key: &char) -> Option<Move> {
    match key {
        'X' => Some(Move::Rock),
        'Y' => Some(Move::Paper),
        'Z' => Some(Move::Scissors),
        _ => None,
    }
}

fn opponent_key_to_move(key: &char) -> Option<Move> {
    match key {
        'A' => Some(Move::Rock),
        'B' => Some(Move::Paper),
        'C' => Some(Move::Scissors),
        _ => None,
    }
}

fn calculate_score(game_str: &str) -> u32 {
    let game_chars: Vec<char> = game_str.chars().collect();
    let player_move: Option<Move> = game_chars.last().and_then(player_key_to_move);
    let opponent_move: Option<Move> = game_chars.first().and_then(opponent_key_to_move);

    match (&player_move, &opponent_move) {
        (Some(player), Some(opponent)) => player.score_against(opponent),
        _ => 0,
    }
}

/// Puzzle 1
pub(super) fn get_total_score_after_rps_games(input_file: &str) -> io::Result<u32> {
    let input_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "input", input_file]
        .iter()
        .collect();

    let file = File::open(input_path)?;
    let buf = BufReader::new(file);

    buf.lines().map(|l| l.map(|s| calculate_score(&s))).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_total_score_after_rps_games_using_example_input() {
        assert_eq!(
            Ok(15),
            get_total_score_after_rps_games("day02-example-input.txt")
                .map_err(|e| format!("{:?}", e))
        );
    }
}
