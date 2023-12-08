advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut window = EngineSchematicWindow::new();

    for line in input.lines().chain("\n\n".lines()) {
        window.add_row_from_line(line);
        window.mark_part_numbers();

        sum += window.sum_row_bottom_part_numbers();
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

struct EngineSchematicWindow {
    overflow_index: usize,
    rows: [EngineSchematicRow; 3],
}

impl EngineSchematicWindow {
    fn new() -> EngineSchematicWindow {
        EngineSchematicWindow {
            overflow_index: 0,
            rows: [
                EngineSchematicRow::new(),
                EngineSchematicRow::new(),
                EngineSchematicRow::new(),
            ],
        }
    }

    fn add_row_from_line(&mut self, line: &str) {
        self.add_row(EngineSchematicRow::from_line(line));
    }

    fn add_row(&mut self, row: EngineSchematicRow) {
        self.rows[self.overflow_index] = row;

        let next_overflow_index = (self.overflow_index + 1) % self.rows.len();
        self.overflow_index = next_overflow_index;
    }

    fn mark_part_numbers(&mut self) {
        let symbol_ranges = self.get_symbol_ranges_from_middle_row();

        if symbol_ranges.is_empty() {
            return;
        }

        for row in self.rows.iter_mut() {
            let mut symbol_iterator = symbol_ranges.iter();
            let mut number_iterator = row.numbers.iter_mut();

            let mut current_symbol = symbol_iterator.next();
            let mut current_number = number_iterator.next();

            while let (Some(symbol), Some(number)) = (&current_symbol, &mut current_number) {
                if number.range.start() > symbol.end() {
                    current_symbol = symbol_iterator.next();
                    continue;
                }

                if symbol.start() > number.range.end() {
                    current_number = number_iterator.next();
                    continue;
                }

                number.mark_as_part_number();
                current_number = number_iterator.next();
            }
        }
    }

    fn get_symbol_ranges_from_middle_row(&self) -> Vec<std::ops::RangeInclusive<usize>> {
        self.get_row_middle()
            .symbol_indices
            .iter()
            .map(|index| index - 1..=index + 1)
            .collect()
    }

    fn get_row_middle(&self) -> &EngineSchematicRow {
        let index = (self.overflow_index + self.rows.len() - 2) % self.rows.len();
        &self.rows[index]
    }

    fn sum_row_bottom_part_numbers(&self) -> u32 {
        self.get_row_bottom()
            .numbers
            .iter()
            .filter(|number| number.is_part_number)
            .map(|number| number.value)
            .sum()
    }

    fn get_row_bottom(&self) -> &EngineSchematicRow {
        &self.rows[self.overflow_index]
    }
}

struct EngineSchematicRow {
    numbers: Vec<EngineSchematicNumber>,
    symbol_indices: Vec<usize>,
}

impl EngineSchematicRow {
    fn from_line(line: &str) -> EngineSchematicRow {
        let mut row = EngineSchematicRow::new();

        for (index, character) in line.chars().enumerate() {
            if character.is_ascii_digit() {
                let value = character.to_digit(10).unwrap();
                let last_number_option = row.numbers.last_mut();

                if let Some(last_number) = last_number_option {
                    if last_number.should_append(index) {
                        last_number.append(value);
                    } else {
                        row.numbers.push(EngineSchematicNumber::new(value, index));
                    }
                } else {
                    row.numbers.push(EngineSchematicNumber::new(value, index));
                }
            } else if character != '.' {
                row.symbol_indices.push(index);
            }
        }

        row
    }

    fn new() -> EngineSchematicRow {
        EngineSchematicRow {
            numbers: Vec::new(),
            symbol_indices: Vec::new(),
        }
    }
}

struct EngineSchematicNumber {
    value: u32,
    range: std::ops::RangeInclusive<usize>,
    is_part_number: bool,
}

impl EngineSchematicNumber {
    fn new(value: u32, index: usize) -> EngineSchematicNumber {
        EngineSchematicNumber {
            value,
            range: index..=index,
            is_part_number: false,
        }
    }

    fn append(&mut self, value: u32) {
        self.value = self.value * 10 + value;
        self.range = *self.range.start()..=self.range.end() + 1;
    }

    fn should_append(&self, index: usize) -> bool {
        index == self.range.end() + 1
    }

    fn mark_as_part_number(&mut self) {
        if !self.is_part_number {
            self.is_part_number = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
