use std::collections::HashMap;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let sum_arrangements = input
        .lines()
        .map(|line| {
            let record = Record::from_line(line);

            record.arrangements_count()
        })
        .sum();

    Some(sum_arrangements)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut cache = HashMap::new();
    let mut records = Vec::new();

    let sum_arrangements = input
        .lines()
        .map(|line| {
            let record = Record::from_line_unfolded(line);

            let arrangements = record.arrangements_count_with_cache(&mut cache);

            records.push(record);

            arrangements
        })
        .sum();

    Some(sum_arrangements)
}

#[derive(Debug)]
struct Record {
    sequence: Vec<Field>,
    fill_counts: Vec<usize>,
}

impl Record {
    fn from_line(line: &str) -> Record {
        let mut line_iterator = line.split_ascii_whitespace();

        Record {
            sequence: line_iterator
                .next()
                .unwrap()
                .bytes()
                .map(Field::from_char)
                .collect(),
            fill_counts: line_iterator
                .next()
                .unwrap()
                .split(',')
                .map(|number_string| number_string.parse().unwrap())
                .collect(),
        }
    }

    fn from_line_unfolded(line: &str) -> Record {
        let mut line_iterator = line.split_ascii_whitespace();
        let sequence_string = line_iterator.next().unwrap();
        let fill_counts_string = line_iterator.next().unwrap();

        let mut sequence = Vec::new();

        for index in 0..5 {
            if index != 0 {
                sequence.push(Field::Unknown);
            }

            sequence.extend(sequence_string.bytes().map(Field::from_char))
        }

        Record {
            sequence,
            fill_counts: (0..5)
                .flat_map(|_| {
                    fill_counts_string
                        .split(',')
                        .map(|number_string| number_string.parse().unwrap())
                })
                .collect(),
        }
    }

    fn arrangements_count(&self) -> usize {
        RecordSlice::from_record(self).arrangements_count()
    }

    fn arrangements_count_with_cache(&self, cache: &mut HashMap<String, usize>) -> usize {
        RecordSlice::from_record(self).arrangements_count_with_cache(cache)
    }
}

struct RecordSlice<'a> {
    sequence: &'a [Field],
    fill_counts: &'a [usize],
}

impl RecordSlice<'_> {
    fn from_record(record: &Record) -> RecordSlice {
        RecordSlice {
            sequence: &record.sequence,
            fill_counts: &record.fill_counts,
        }
    }

    fn arrangements_count(&self) -> usize {
        if let Some(&next_fill_count) = self.next_fill_count() {
            let mut arrangements_count = 0;

            for index in 0..=self.index_end_for_next_fill_count() {
                let field = &self.sequence[index];

                if field.is_empty() {
                    continue;
                }

                let sub_arrangements_count = self
                    .fill(index, next_fill_count)
                    .map_or(0, |record_slice| record_slice.arrangements_count());

                arrangements_count += sub_arrangements_count;

                if field.is_filled() {
                    break;
                }
            }

            arrangements_count
        } else if self.sequence.iter().all(|field| field.can_be_empty()) {
            1
        } else {
            0
        }
    }

    fn arrangements_count_with_cache(&self, cache: &mut HashMap<String, usize>) -> usize {
        if let Some(&next_fill_count) = self.next_fill_count() {
            let mut arrangements_count = 0;

            for index in 0..=self.index_end_for_next_fill_count() {
                let field = &self.sequence[index];

                if field.is_empty() {
                    continue;
                }

                let sub_arrangements_count =
                    self.fill(index, next_fill_count).map_or(0, |record_slice| {
                        let key = record_slice.to_key();

                        if let Some(&arrangements) = cache.get(&key) {
                            arrangements
                        } else {
                            let arrangements = record_slice.arrangements_count_with_cache(cache);

                            cache.insert(key, arrangements);

                            arrangements
                        }
                    });

                arrangements_count += sub_arrangements_count;

                if field.is_filled() {
                    break;
                }
            }

            arrangements_count
        } else if self.sequence.iter().all(|field| field.can_be_empty()) {
            1
        } else {
            0
        }
    }

    fn next_fill_count(&self) -> Option<&usize> {
        self.fill_counts.first()
    }

    fn index_end_for_next_fill_count(&self) -> usize {
        let fill_counts_count = self.fill_counts.len();
        let fields_count = self.sequence.len();

        if fill_counts_count == 0 {
            return fields_count;
        }

        fields_count - self.fill_counts.iter().sum::<usize>() - (fill_counts_count - 1)
    }

    fn fill(&self, index: usize, fill_count: usize) -> Option<RecordSlice> {
        let index_end = index + fill_count;

        if self.sequence[index..index_end]
            .iter()
            .all(|field| field.can_be_filled())
        {
            if let Some(field) = self.sequence.get(index_end) {
                if field.can_be_empty() {
                    Some(self.next(index_end + 1))
                } else {
                    None
                }
            } else {
                Some(self.next(index_end))
            }
        } else {
            None
        }
    }

    fn next(&self, index: usize) -> RecordSlice {
        RecordSlice {
            sequence: &self.sequence[index..],
            fill_counts: &self.fill_counts[1..],
        }
    }

    fn to_key(&self) -> String {
        let mut key = String::new();

        for field in self.sequence {
            key.push(field.to_char());
        }

        for fill_count in self.fill_counts {
            key.push_str(&fill_count.to_string());
        }

        key
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Field {
    Empty,
    Filled,
    Unknown,
}

impl Field {
    fn from_char(char: u8) -> Self {
        match char {
            b'#' => Field::Filled,
            b'.' => Field::Empty,
            b'?' => Field::Unknown,
            _ => panic!("Invalid field char {}", char as char),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Field::Filled => '#',
            Field::Empty => '.',
            Field::Unknown => '?',
        }
    }

    fn is_empty(&self) -> bool {
        *self == Field::Empty
    }

    fn is_filled(&self) -> bool {
        *self == Field::Filled
    }

    fn is_unknown(&self) -> bool {
        *self == Field::Unknown
    }

    fn can_be_empty(&self) -> bool {
        self.is_empty() || self.is_unknown()
    }

    fn can_be_filled(&self) -> bool {
        self.is_filled() || self.is_unknown()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
