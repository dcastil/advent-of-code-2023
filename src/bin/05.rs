use std::cmp::Ordering;
use std::ops::Range;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i64> {
    let mut input_iterator = input.split("\n\n");

    let mut current_values: Vec<i64> = parse_seed_values(input_iterator.next().unwrap());

    for category_string in input_iterator {
        let category_map = CategoryMap::from_string(category_string);

        for value in current_values.iter_mut() {
            *value = category_map.map_value(*value);
        }
    }

    current_values.iter().min().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_seed_values(line: &str) -> Vec<i64> {
    line.split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|seed_string| seed_string.parse::<i64>().unwrap())
        .collect()
}

struct CategoryMap {
    value_maps_sorted: Vec<ValueMap>,
}

impl CategoryMap {
    fn from_string(string: &str) -> CategoryMap {
        let mut category_map = CategoryMap::new();

        for line in string.lines().skip(1) {
            category_map
                .value_maps_sorted
                .push(ValueMap::from_line(line));
        }

        category_map
            .value_maps_sorted
            .sort_by(|a, b| a.source_range.start.cmp(&b.source_range.start));

        category_map
    }

    fn new() -> CategoryMap {
        CategoryMap {
            value_maps_sorted: Vec::new(),
        }
    }

    fn map_value(&self, value: i64) -> i64 {
        if let Ok(index) = self.value_maps_sorted.binary_search_by(|value_map| {
            if value_map.source_range.start > value {
                Ordering::Greater
            } else if value_map.source_range.end <= value {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }) {
            return value + self.value_maps_sorted[index].translate_by;
        }

        value
    }
}

struct ValueMap {
    source_range: Range<i64>,
    translate_by: i64,
}

impl ValueMap {
    fn from_line(line: &str) -> ValueMap {
        let mut line_iterator = line.split(' ');

        let destination_range_start: i64 = line_iterator.next().unwrap().parse().unwrap();
        let source_range_start: i64 = line_iterator.next().unwrap().parse().unwrap();
        let range_length: i64 = line_iterator.next().unwrap().parse().unwrap();

        ValueMap {
            source_range: source_range_start..source_range_start + range_length,
            translate_by: destination_range_start - source_range_start,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
