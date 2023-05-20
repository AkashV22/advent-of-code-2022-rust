use std::error::Error;

mod day01;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Day 1 puzzle 1 result: {}",
        day01::get_total_calories_for_elf_with_most_calories("day01-input.txt")?
    );

    println!(
        "Day 1 puzzle 2 result: {}",
        day01::get_total_calories_for_three_elves_with_most_calories("day01-input.txt")?
    );

    Ok(())
}
