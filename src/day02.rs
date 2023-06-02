use crate::shared::{InputLoader, Transposable};
use std::io;

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

    fn from_player_key(key: &char) -> Option<Self> {
        match key {
            'X' => Some(Self::Lose),
            'Y' => Some(Self::Draw),
            'Z' => Some(Self::Win),
            _ => None,
        }
    }
}

#[derive(Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_player_key(key: &char) -> Option<Self> {
        match key {
            'X' => Some(Self::Rock),
            'Y' => Some(Self::Paper),
            'Z' => Some(Self::Scissors),
            _ => None,
        }
    }

    fn from_opponent_key(key: &char) -> Option<Self> {
        match key {
            'A' => Some(Self::Rock),
            'B' => Some(Self::Paper),
            'C' => Some(Self::Scissors),
            _ => None,
        }
    }

    fn from(player_result: &GameResult, opponent_move: &Self) -> Self {
        match (player_result, opponent_move) {
            (GameResult::Lose, Self::Rock) => Self::Scissors,
            (GameResult::Lose, Self::Paper) => Self::Rock,
            (GameResult::Lose, Self::Scissors) => Self::Paper,
            (GameResult::Draw, opponent_move) => opponent_move.clone(),
            (GameResult::Win, Self::Rock) => Self::Paper,
            (GameResult::Win, Self::Paper) => Self::Scissors,
            (GameResult::Win, Self::Scissors) => Self::Rock,
        }
    }

    fn score_against(&self, opponent: &Self) -> u32 {
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

/// Puzzle 1
pub(super) fn get_total_score_after_rps_games_with_player_moves(
    input_file: &str,
) -> io::Result<u32> {
    input_file
        .load_input_to_lines()?
        .map(|r| r.map(|s| s.chars().collect::<Vec<char>>()))
        .map(|r| {
            r.map(|game_chars| {
                (
                    game_chars.last().and_then(Move::from_player_key),
                    game_chars.first().and_then(Move::from_opponent_key),
                )
            })
        })
        .map(|r| r.map(|moves| moves.transpose()))
        .map(|r| r.map(|moves| moves.map(|(player, opponent)| player.score_against(&opponent))))
        .map(|r| r.map(|score| score.unwrap_or_default()))
        .sum()
}

/// Puzzle 2
pub(super) fn get_total_score_after_rps_games_with_player_results(
    input_file: &str,
) -> io::Result<u32> {
    input_file
        .load_input_to_lines()?
        .map(|r| r.map(|s| s.chars().collect::<Vec<char>>()))
        .map(|r| {
            r.map(|game_chars| {
                (
                    game_chars.last().and_then(GameResult::from_player_key),
                    game_chars.first().and_then(Move::from_opponent_key),
                )
            })
        })
        .map(|r| r.map(|result_and_move| result_and_move.transpose()))
        .map(|r| {
            r.map(|result_and_move| {
                result_and_move.map(|(player_result, opponent_move)| {
                    (Move::from(&player_result, &opponent_move), opponent_move)
                })
            })
        })
        .map(|r| r.map(|moves| moves.map(|(player, opponent)| player.score_against(&opponent))))
        .map(|r| r.map(|score| score.unwrap_or_default()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_total_score_after_rps_games_with_player_moves_using_example_input() {
        assert_eq!(
            Ok(15),
            get_total_score_after_rps_games_with_player_moves("day02-example-input.txt")
                .map_err(|e| format!("{:?}", e))
        );
    }

    #[test]
    fn get_total_score_after_rps_games_with_player_results_using_example_input() {
        assert_eq!(
            Ok(12),
            get_total_score_after_rps_games_with_player_results("day02-example-input.txt")
                .map_err(|e| format!("{:?}", e))
        );
    }
}
