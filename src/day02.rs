use crate::shared::InputLoader;
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
}

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

trait Transposable<R> {
    fn transpose(self) -> R;
}

impl<T, U> Transposable<Option<(T, U)>> for (Option<T>, Option<U>) {
    fn transpose(self) -> Option<(T, U)> {
        match self {
            (Some(first), Some(second)) => Some((first, second)),
            _ => None,
        }
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
}
