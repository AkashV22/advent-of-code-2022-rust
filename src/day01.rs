use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn get_total_calories_for_elves_with_most_calories(
    input_file: &str,
    limit: usize,
) -> Result<u32, Box<dyn Error>> {
    let input_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "input", input_file]
        .iter()
        .collect();

    let file = File::open(input_path)?;
    let buf = BufReader::new(file);

    let mut calories_for_current_elf: u32 = 0;
    let mut calories_for_all_elves: Vec<u32> = vec![];
    for line in buf.lines() {
        let calories_str: &str = &line?;
        if !calories_str.is_empty() {
            calories_for_current_elf += calories_str.parse::<u32>()?;
            continue;
        }
        calories_for_all_elves.push(calories_for_current_elf);
        calories_for_current_elf = 0;
    }

    if calories_for_current_elf > 0 {
        calories_for_all_elves.push(calories_for_current_elf);
    }

    calories_for_all_elves.sort_by(|x, y| y.cmp(x));

    Ok(calories_for_all_elves.iter().take(limit).sum())
}

/// Puzzle 1
pub(super) fn get_total_calories_for_elf_with_most_calories(
    input_file: &str,
) -> Result<u32, Box<dyn Error>> {
    get_total_calories_for_elves_with_most_calories(input_file, 1)
}

/// Puzzle 2
pub(super) fn get_total_calories_for_three_elves_with_most_calories(
    input_file: &str,
) -> Result<u32, Box<dyn Error>> {
    get_total_calories_for_elves_with_most_calories(input_file, 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_total_calories_for_elf_with_most_calories_using_example_input() {
        assert_eq!(
            Ok(24000),
            get_total_calories_for_elf_with_most_calories("day01-example-input.txt")
                .map_err(|e| format!("{:?}", e))
        );
    }

    #[test]
    fn get_total_calories_for_three_elves_with_most_calories_using_example_input() {
        assert_eq!(
            Ok(45000),
            get_total_calories_for_three_elves_with_most_calories("day01-example-input.txt")
                .map_err(|e| format!("{:?}", e))
        );
    }
}
