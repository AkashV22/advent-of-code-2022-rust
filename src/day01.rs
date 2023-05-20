use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

/// Puzzle 1
pub(super) fn get_total_calories_for_elf_with_most_calories(
    input_file: &str,
) -> Result<u32, String> {
    let input_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "input", input_file]
        .iter()
        .collect();

    let file = File::open(input_path).map_err(|e| format!("{:?}", e))?;
    let buf = BufReader::new(file);

    let mut max_calories: u32 = 0;
    let mut calories_for_current_elf: u32 = 0;
    for line in buf.lines() {
        let calories_str: &str = &line.map_err(|e| format!("{:?}", e))?;
        if !calories_str.is_empty() {
            calories_for_current_elf += calories_str
                .parse::<u32>()
                .map_err(|e| format!("{:?}", e))?;
            continue;
        }
        if calories_for_current_elf > max_calories {
            max_calories = calories_for_current_elf;
        }
        calories_for_current_elf = 0;
    }

    Ok(max_calories)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_using_example_input() {
        assert_eq!(
            Ok(24000),
            get_total_calories_for_elf_with_most_calories("day01-example-input.txt")
        );
    }
}
