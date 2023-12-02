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
    None
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
        assert_eq!(result, None);
    }
}
