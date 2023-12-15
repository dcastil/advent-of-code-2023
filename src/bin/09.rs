advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let mut sum = 0;

    for line in input.lines() {
        let interpolated_value = interpolate_sequence(line, Direction::Forward);

        sum += interpolated_value;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut sum = 0;

    for line in input.lines() {
        let interpolated_value = interpolate_sequence(line, Direction::Backward);

        sum += interpolated_value;
    }

    Some(sum)
}

enum Direction {
    Forward,
    Backward,
}

fn interpolate_sequence(line: &str, direction: Direction) -> i32 {
    let mut levels = Vec::new();

    let mut current_values = parse_values(line);

    while !values_are_zero(&current_values) {
        let next_values = get_derivatives(&current_values);

        levels.push(current_values);

        current_values = next_values;
    }

    let mut interpolated_value = 0;

    for values in levels.iter().rev() {
        interpolated_value = match direction {
            Direction::Forward => values.last().unwrap() + interpolated_value,
            Direction::Backward => values.first().unwrap() - interpolated_value,
        }
    }

    interpolated_value
}

fn parse_values(line: &str) -> Vec<i32> {
    line.split(' ').map(|s| s.parse().unwrap()).collect()
}

fn get_derivatives(values: &[i32]) -> Vec<i32> {
    values
        .iter()
        .zip(values.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect()
}

fn values_are_zero(values: &[i32]) -> bool {
    values.iter().all(|value| *value == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
