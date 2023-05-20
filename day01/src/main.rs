use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn get_total_calories_for_elf_with_most_calories(input_file: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let input_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), input_file].iter().collect();

    let file = File::open(input_path)?;
    let buf = BufReader::new(file);

    let mut max_calories: u32 = 0;
    let mut calories_for_current_elf: u32 = 0;
    for line in buf.lines() {
        let calories_str: &str = &line?;
        if calories_str.is_empty() {
            if calories_for_current_elf > max_calories {
                max_calories = calories_for_current_elf;
            }
            calories_for_current_elf = 0;
        } else {
            calories_for_current_elf += calories_str.parse::<u32>()?;
        }
    }

    Ok(max_calories)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let max_calories: u32 = get_total_calories_for_elf_with_most_calories("input.txt")?;

    println!("The elf carrying the most calories is carrying {} calories in total!", max_calories);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_using_example_input() {
        assert_eq!(Ok(24000), get_total_calories_for_elf_with_most_calories("example-input.txt").map_err(|e| format!("{:?}", e)));
    }
}
