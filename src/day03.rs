use crate::shared::InputLoader;
use std::collections::HashMap;
use std::error::Error;

fn build_priority_map(
    priority_map: &mut HashMap<char, u8>,
    first_ch: u8,
    last_ch: u8,
    lowest_priority: u8,
) {
    let chars = (first_ch..=last_ch)
        .map(|c| c as char)
        .filter(|ch| ch.is_alphabetic())
        .collect::<Vec<_>>();

    for (i, ch) in chars.iter().enumerate() {
        priority_map.insert(*ch, i as u8 + lowest_priority);
    }
}

/// Puzzle 1
pub(crate) fn get_sum_of_backpack_errors(input_file: &str) -> Result<u32, Box<dyn Error>> {
    let mut priority_map: HashMap<char, u8> = HashMap::new();
    build_priority_map(&mut priority_map, b'a', b'z', 1);
    build_priority_map(&mut priority_map, b'A', b'Z', 27);

    let mut sum: u32 = 0;
    for line in input_file.load_input_to_lines()? {
        let backpack: String = line?;
        let (first_compartment, second_compartment) = backpack.split_at(backpack.len() / 2);
        for first_compartment_ch in first_compartment.chars() {
            let mut priority_found = false;
            for second_compartment_ch in second_compartment.chars() {
                if first_compartment_ch != second_compartment_ch {
                    continue;
                }
                if let Some(priority) = priority_map.get(&first_compartment_ch) {
                    sum += *priority as u32;
                    priority_found = true;
                    break;
                }
            }
            if priority_found {
                break;
            }
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_sum_of_backpack_errors_using_example_input() {
        assert_eq!(
            Ok(157),
            get_sum_of_backpack_errors("day03-example-input.txt").map_err(|e| format!("{:?}", e))
        );
    }
}
