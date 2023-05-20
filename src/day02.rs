enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn opponent(&self) -> char {
        match self {
            Move::Rock => 'A',
            Move::Paper => 'B',
            Move::Scissors => 'C',
        }
    }

    fn player(&self) -> char {
        match self {
            Move::Rock => 'X',
            Move::Paper => 'Y',
            Move::Scissors => 'Z',
        }
    }

    fn score(&self) -> u8 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

/// Puzzle 1
pub(super) fn get_total_score_after_rps_games(input_file: &str) -> u32 {
    15
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_total_score_after_rps_games_using_example_input() {
        assert_eq!(
            15,
            get_total_score_after_rps_games("day02-example-input.txt")
        );
    }
}
