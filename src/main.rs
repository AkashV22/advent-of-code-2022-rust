mod day01;

fn main() -> Result<(), String> {
    println!(
        "Day 1 exercise 1 result: {}",
        day01::get_total_calories_for_elf_with_most_calories("day01-input.txt")?
    );

    Ok(())
}
