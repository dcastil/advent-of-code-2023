advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let first_number = line
            .chars()
            .find(|&c| c.is_numeric())
            .map(|c| c.to_digit(10).unwrap());
        let last_number = line
            .chars()
            .rev()
            .find(|&c| c.is_numeric())
            .map(|c| c.to_digit(10).unwrap());

        if let (Some(first), Some(last)) = (first_number, last_number) {
            let number = first * 10 + last;

            sum += number;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let number_strings = [
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
    ];

    let mut sum = 0;

    for line in input.lines() {
        let string_positions = get_string_positions(line, &number_strings);

        let mut first_position = get_first_numeric_position(line);
        let mut last_position = get_last_numeric_position(line);

        for (position_first, position_last) in string_positions {
            if let Some(position) = position_first {
                if let Some(ref first) = first_position {
                    if position.index < first.index {
                        let position = Some(position);
                        first_position = position;
                    }
                } else {
                    first_position = Some(position);
                }
            }

            if let Some(position) = position_last {
                if let Some(ref last) = last_position {
                    if position.index > last.index {
                        last_position = Some(position);
                    }
                } else {
                    last_position = Some(position);
                }
            }
        }

        if let (Some(first), Some(last)) = (first_position, last_position) {
            let number = first.number * 10 + last.number;

            sum += number;
        }
    }

    Some(sum)
}

fn get_string_positions<const N: usize>(
    line: &str,
    number_strings: &[(u32, &str); N],
) -> [(Option<NumberPosition>, Option<NumberPosition>); N] {
    return number_strings.map(|(number, number_string)| {
        let position_first = if let Some(index) = line.find(number_string) {
            Some(NumberPosition { number, index })
        } else {
            None
        };

        let position_last = if let Some(index) = line.rfind(number_string) {
            Some(NumberPosition { number, index })
        } else {
            None
        };

        (position_first, position_last)
    });
}

fn get_first_numeric_position(line: &str) -> Option<NumberPosition> {
    for (index, character) in line.chars().enumerate() {
        if character.is_numeric() {
            return Some(NumberPosition {
                number: character.to_digit(10).unwrap(),
                index,
            });
        }
    }

    None
}

fn get_last_numeric_position(line: &str) -> Option<NumberPosition> {
    for (index, character) in line.chars().rev().enumerate() {
        if character.is_numeric() {
            return Some(NumberPosition {
                number: character.to_digit(10).unwrap(),
                index: line.len() - 1 - index,
            });
        }
    }

    None
}

struct NumberPosition {
    number: u32,
    index: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
