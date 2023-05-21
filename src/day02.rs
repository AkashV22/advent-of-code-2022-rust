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
            GameResult::Win => 6,
            GameResult::Lose => 0,
            GameResult::Draw => 3,
        }
    }
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn result_against(&self, opponent: &Move) -> GameResult {
        match (self, opponent) {
            (Move::Rock, Move::Rock) => GameResult::Draw,
            (Move::Rock, Move::Paper) => GameResult::Lose,
            (Move::Rock, Move::Scissors) => GameResult::Win,
            (Move::Paper, Move::Rock) => GameResult::Win,
            (Move::Paper, Move::Paper) => GameResult::Draw,
            (Move::Paper, Move::Scissors) => GameResult::Lose,
            (Move::Scissors, Move::Rock) => GameResult::Lose,
            (Move::Scissors, Move::Paper) => GameResult::Win,
            (Move::Scissors, Move::Scissors) => GameResult::Draw,
        }
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
        (Some(player), Some(opponent)) => player.score() + player.result_against(opponent).score(),
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
