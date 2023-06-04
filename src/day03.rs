use crate::shared::InputLoader;
use std::collections::HashMap;
use std::error::Error;

trait PriorityAdder {
    fn add_priorities(self, first_ch: u8, last_ch: u8, lowest_priority: u8) -> Self;
}

impl PriorityAdder for HashMap<char, u8> {
    fn add_priorities(mut self, first_ch: u8, last_ch: u8, lowest_priority: u8) -> Self {
        let chars = (first_ch..=last_ch)
            .map(|c| c as char)
            .filter(|ch| ch.is_alphabetic())
            .collect::<Vec<_>>();

        for (i, ch) in chars.iter().enumerate() {
            self.insert(*ch, i as u8 + lowest_priority);
        }
        self
    }
}

/// Puzzle 1
pub(crate) fn get_sum_of_backpack_errors(input_file: &str) -> Result<u32, Box<dyn Error>> {
    let priority_map: HashMap<char, u8> = HashMap::new()
        .add_priorities(b'a', b'z', 1)
        .add_priorities(b'A', b'Z', 27);

    let mut sum: u32 = 0;
    for line in input_file.load_input_to_lines()? {
        let backpack: String = line?;
        let (first_compartment, second_compartment) = backpack.split_at(backpack.len() / 2);

        let mut find_backpack_errors_and_add_priority_to_sum = || {
            for first_compartment_ch in first_compartment.chars() {
                for second_compartment_ch in second_compartment.chars() {
                    if first_compartment_ch != second_compartment_ch {
                        continue;
                    }
                    if let Some(priority) = priority_map.get(&first_compartment_ch) {
                        sum += *priority as u32;
                        return;
                    }
                }
            }
        };
        find_backpack_errors_and_add_priority_to_sum();
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
