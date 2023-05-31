use std::error::Error;

mod day01;
mod day02;
mod shared;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Day 1 puzzle 1 result: {}",
        day01::get_total_calories_for_elf_with_most_calories("day01-input.txt")?
    );

    println!(
        "Day 1 puzzle 2 result: {}",
        day01::get_total_calories_for_three_elves_with_most_calories("day01-input.txt")?
    );

    println!(
        "Day 2 puzzle 1 result: {}",
        day02::get_total_score_after_rps_games_with_player_moves("day02-input.txt")?
    );

    println!(
        "Day 2 puzzle 2 result: {}",
        day02::get_total_score_after_rps_games_with_player_results("day02-input.txt")?
    );

    Ok(())
}
