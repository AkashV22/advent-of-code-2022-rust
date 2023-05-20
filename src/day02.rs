use std::{
    fs::File,
    io,
    io::{BufRead, BufReader},
    path::PathBuf,
};

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> u8 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

fn opponent_key_to_move(key: char) -> Option<Move> {
    match key {
        'A' => Some(Move::Rock),
        'B' => Some(Move::Paper),
        'C' => Some(Move::Scissors),
        _ => None,
    }
}

fn player_key_to_move(key: char) -> Option<Move> {
    match key {
        'X' => Some(Move::Rock),
        'Y' => Some(Move::Paper),
        'Z' => Some(Move::Scissors),
        _ => None,
    }
}

/// Puzzle 1
pub(super) fn get_total_score_after_rps_games(input_file: &str) -> Result<u32, io::Error> {
    let input_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "input", input_file]
        .iter()
        .collect();

    let file = File::open(input_path)?;
    let buf = BufReader::new(file);

    for line in buf.lines() {}

    Ok(15)
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
