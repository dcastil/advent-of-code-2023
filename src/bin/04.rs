use regex::Regex;
use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let amount_of_wins = parse_amount_of_wins(line);

        if amount_of_wins != 0 {
            sum += 1 << (amount_of_wins - 1);
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_amount_of_wins(line: &str) -> u32 {
    let line_regex = Regex::new(r"^Card\s+\d+?: ([\d\s]+\d)\s+\|\s+(\d[\d\s]+)$").unwrap();
    let space_regex = Regex::new(r"\s+").unwrap();

    let caps = line_regex.captures(line).unwrap();

    let winning_numbers_set: HashSet<&str> = space_regex.split(&caps[1]).collect();
    let posessed_numbers_set: HashSet<&str> = space_regex.split(&caps[2]).collect();

    let amount_of_wins = winning_numbers_set
        .intersection(&posessed_numbers_set)
        .count();

    amount_of_wins as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
