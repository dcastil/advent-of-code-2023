advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    let sum = input
        .split("\n\n")
        .map(|pattern_string| {
            let matrix = PatternMatrix::from_pattern_string(pattern_string);

            match matrix.symmetry_with_exceptions(0) {
                Symmetry::Vertical { index, .. } => index,
                Symmetry::Horizontal { index, .. } => index * 100,
            }
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let sum = input
        .split("\n\n")
        .map(|pattern_string| {
            let matrix = PatternMatrix::from_pattern_string(pattern_string);

            match matrix.symmetry_with_exceptions(1) {
                Symmetry::Vertical { index, .. } => index,
                Symmetry::Horizontal { index, .. } => index * 100,
            }
        })
        .sum();

    Some(sum)
}

struct PatternMatrix {
    matrix: Vec<Vec<Pattern>>,
}

impl PatternMatrix {
    fn from_pattern_string(pattern_string: &str) -> PatternMatrix {
        let matrix = pattern_string
            .lines()
            .map(|line| line.chars().map(Pattern::from_char).collect())
            .collect();

        PatternMatrix { matrix }
    }

    fn symmetry_with_exceptions(&self, exceptions_count: usize) -> Symmetry {
        let mut symmetries_vertical = self.possible_symmetries_vertical();

        for index in 0..self.length_vertical() {
            symmetries_vertical.retain_mut(|symmetry| {
                self.check_symmetry_at_line(symmetry, index);

                symmetry.exceptions_count() <= exceptions_count
            });

            if symmetries_vertical.is_empty() {
                break;
            }
        }

        if let Some(symmetry) = symmetries_vertical
            .into_iter()
            .find(|symmetry| symmetry.exceptions_count() == exceptions_count)
        {
            return symmetry;
        }

        let mut symmetries_horizontal = self.possible_symmetries_horizontal();

        for index in 0..self.length_horizontal() {
            symmetries_horizontal.retain_mut(|symmetry| {
                self.check_symmetry_at_line(symmetry, index);

                symmetry.exceptions_count() <= exceptions_count
            });

            if symmetries_horizontal.is_empty() {
                break;
            }
        }

        if let Some(symmetry) = symmetries_horizontal
            .into_iter()
            .find(|symmetry| symmetry.exceptions_count() == exceptions_count)
        {
            return symmetry;
        }

        panic!("No symmetry found");
    }

    fn check_symmetry_at_line(&self, symmetry: &mut Symmetry, line_index: usize) {
        let range_end = match *symmetry {
            Symmetry::Vertical { index, .. } => index.min(self.length_horizontal() - index),
            Symmetry::Horizontal { index, .. } => index.min(self.length_vertical() - index),
        };

        for distance in 0..range_end {
            if !self.has_symmetry_at_index(symmetry, line_index, distance) {
                symmetry.increment_exceptions_count();
            }
        }
    }

    fn has_symmetry_at_index(
        &self,
        symmetry: &Symmetry,
        line_index: usize,
        distance: usize,
    ) -> bool {
        match *symmetry {
            Symmetry::Vertical { index, .. } => {
                self.matrix[line_index][index - distance - 1]
                    == self.matrix[line_index][index + distance]
            }
            Symmetry::Horizontal { index, .. } => {
                self.matrix[index - distance - 1][line_index]
                    == self.matrix[index + distance][line_index]
            }
        }
    }

    fn possible_symmetries_vertical(&self) -> Vec<Symmetry> {
        (1..self.length_horizontal())
            .map(Symmetry::vertical)
            .collect()
    }

    fn possible_symmetries_horizontal(&self) -> Vec<Symmetry> {
        (1..self.length_vertical())
            .map(Symmetry::horizontal)
            .collect()
    }

    fn length_vertical(&self) -> usize {
        self.matrix.len()
    }

    fn length_horizontal(&self) -> usize {
        // Assuming all rows have the same length
        self.matrix[0].len()
    }
}

#[derive(PartialEq, Debug)]
enum Pattern {
    Hash,
    Dot,
}

impl Pattern {
    fn from_char(character: char) -> Pattern {
        match character {
            '#' => Pattern::Hash,
            '.' => Pattern::Dot,
            _ => panic!("Invalid pattern character {}", character),
        }
    }
}

#[derive(Clone, Debug)]
enum Symmetry {
    Vertical {
        index: usize,
        exceptions_count: usize,
    },
    Horizontal {
        index: usize,
        exceptions_count: usize,
    },
}

impl Symmetry {
    fn vertical(index: usize) -> Symmetry {
        Symmetry::Vertical {
            index,
            exceptions_count: 0,
        }
    }

    fn horizontal(index: usize) -> Symmetry {
        Symmetry::Horizontal {
            index,
            exceptions_count: 0,
        }
    }

    fn exceptions_count(&self) -> usize {
        match self {
            Symmetry::Vertical {
                exceptions_count, ..
            } => *exceptions_count,
            Symmetry::Horizontal {
                exceptions_count, ..
            } => *exceptions_count,
        }
    }

    fn increment_exceptions_count(&mut self) {
        match self {
            Symmetry::Vertical {
                exceptions_count, ..
            } => *exceptions_count += 1,
            Symmetry::Horizontal {
                exceptions_count, ..
            } => *exceptions_count += 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
