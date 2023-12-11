use std::cmp::{self, Ordering};
use std::ops::Range;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i64> {
    let mut input_iterator = input.split("\n\n");
    let mut current_values = parse_seed_values(input_iterator.next().unwrap());

    for category_string in input_iterator {
        let category_map = CategoryMap::from_string(category_string);

        for value in current_values.iter_mut() {
            *value = category_map.map_value(*value);
        }
    }

    current_values.iter().min().copied()
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut input_iterator = input.split("\n\n");
    let mut current_ranges = parse_seed_ranges(input_iterator.next().unwrap());

    for category_string in input_iterator {
        let category_map = CategoryMap::from_string(category_string);

        current_ranges = current_ranges
            .iter()
            .flat_map(|range| category_map.map_range(range))
            .collect();
    }

    current_ranges.iter().map(|range| range.start).min()
}

fn parse_seed_values(line: &str) -> Vec<i64> {
    line.split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|seed_string| seed_string.parse::<i64>().unwrap())
        .collect()
}

fn parse_seed_ranges(line: &str) -> Vec<Range<i64>> {
    let seeds_split = line.split(": ").nth(1).unwrap().split(' ');

    seeds_split
        .clone()
        .step_by(2)
        .zip(seeds_split.skip(1).step_by(2))
        .map(|(start_string, length_string)| {
            let start: i64 = start_string.parse().unwrap();
            let length: i64 = length_string.parse().unwrap();

            start..start + length
        })
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
        if let Ok(index) = self.bindary_search_value_map(value) {
            return value + self.value_maps_sorted[index].translate_by;
        }

        value
    }

    fn bindary_search_value_map(&self, value: i64) -> Result<usize, usize> {
        self.value_maps_sorted.binary_search_by(|value_map| {
            if value_map.source_range.start > value {
                Ordering::Greater
            } else if value_map.source_range.end <= value {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        })
    }

    fn map_range(&self, range: &Range<i64>) -> Vec<Range<i64>> {
        let value_maps_slice_start = self
            .bindary_search_value_map(range.start)
            .unwrap_or_else(|index| index);
        let value_maps_slice_end = match self.bindary_search_value_map(range.end - 1) {
            Ok(index) => index + 1,
            Err(index) => index,
        };

        let value_maps = &self.value_maps_sorted[value_maps_slice_start..value_maps_slice_end];

        let mut next_ranges = Vec::new();
        let mut current_value = range.start;

        for value_map in value_maps {
            if current_value < value_map.source_range.start {
                next_ranges.push(current_value..value_map.source_range.start);

                current_value = value_map.source_range.start;
            }

            let next_value = cmp::min(value_map.source_range.end, range.end);

            next_ranges
                .push(current_value + value_map.translate_by..next_value + value_map.translate_by);

            current_value = next_value;
        }

        if current_value < range.end {
            next_ranges.push(current_value..range.end);
        }

        next_ranges
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
        assert_eq!(result, Some(46));
    }
}
