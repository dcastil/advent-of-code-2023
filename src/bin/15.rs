use std::collections::HashMap;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.trim().split(',').map(hash_string).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut boxes: HashMap<u32, Vec<(&str, u32)>> = HashMap::new();

    for string in input.trim().split(',') {
        if *string.as_bytes().last().unwrap() == b'-' {
            let lens = &string[..string.len() - 1];
            let box_number = hash_string(lens);

            if let Some(values) = boxes.get_mut(&box_number) {
                if let Some(index) = values
                    .iter()
                    .position(|(current_lens, ..)| *current_lens == lens)
                {
                    values.remove(index);
                }
            }
        } else {
            let lens = &string[..string.len() - 2];
            let value = string.chars().last().unwrap().to_digit(10).unwrap();
            let box_number = hash_string(lens);

            let values = boxes.entry(box_number).or_default();

            if let Some((_, value_in_map)) = values
                .iter_mut()
                .find(|(current_lens, ..)| *current_lens == lens)
            {
                *value_in_map = value;
            } else {
                values.push((lens, value));
            }
        }
    }

    Some(
        boxes
            .into_iter()
            .flat_map(|(box_number, values)| {
                values
                    .into_iter()
                    .enumerate()
                    .map(move |(index, (_, value))| (box_number + 1) * (index as u32 + 1) * value)
            })
            .sum(),
    )
}

fn hash_string(string: &str) -> u32 {
    string
        .bytes()
        .fold(0, |hash, character| ((hash + character as u32) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
