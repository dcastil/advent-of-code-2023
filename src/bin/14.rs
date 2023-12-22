advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let mut rocks_per_line = Vec::new();

    // Assuming all lines have the same length
    let line_length = input.find('\n').unwrap();
    let mut next_empty_slot_per_column = vec![0; line_length];

    for (line_index, line) in input.lines().enumerate() {
        rocks_per_line.push(0);

        for (column_index, character) in line.chars().enumerate() {
            if character == '#' {
                next_empty_slot_per_column[column_index] = line_index + 1;
            } else if character == 'O' {
                rocks_per_line[next_empty_slot_per_column[column_index]] += 1;
                next_empty_slot_per_column[column_index] += 1;
            }
        }
    }

    let total_load = rocks_per_line
        .iter()
        .rev()
        .enumerate()
        .fold(0, |sum, (index, rocks_count)| {
            sum + rocks_count * (index + 1)
        });

    Some(total_load)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
