advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    let sum = input
        .split("\n\n")
        .map(|pattern_string| {
            let matrix = PatternMatrix::from_pattern_string(pattern_string);

            match matrix.symmetry() {
                Symmetry::Vertical(index) => index,
                Symmetry::Horizontal(index) => index * 100,
            }
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
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

    fn symmetry(&self) -> Symmetry {
        let mut symmetries_vertical = self.possible_symmetries_vertical();

        for index in 0..self.length_vertical() {
            symmetries_vertical.retain(|symmetry| self.has_symmetry_at_line(symmetry, index));

            if symmetries_vertical.is_empty() {
                break;
            }
        }

        if let Some(symmetry) = symmetries_vertical.first() {
            return symmetry.clone();
        }

        let mut symmetries_horizontal = self.possible_symmetries_horizontal();

        for index in 0..self.length_horizontal() {
            symmetries_horizontal.retain(|symmetry| self.has_symmetry_at_line(symmetry, index));

            if symmetries_horizontal.is_empty() {
                break;
            }
        }

        if let Some(symmetry) = symmetries_horizontal.first() {
            return symmetry.clone();
        }

        panic!("No symmetry found");
    }

    fn has_symmetry_at_line(&self, symmetry: &Symmetry, line_index: usize) -> bool {
        let range_end = match *symmetry {
            Symmetry::Vertical(index) => index.min(self.length_horizontal() - index),
            Symmetry::Horizontal(index) => index.min(self.length_vertical() - index),
        };

        (0..range_end).all(|distance| self.has_symmetry_at_index(symmetry, line_index, distance))
    }

    fn has_symmetry_at_index(
        &self,
        symmetry: &Symmetry,
        line_index: usize,
        distance: usize,
    ) -> bool {
        match *symmetry {
            Symmetry::Vertical(index) => {
                self.matrix[line_index][index - distance - 1]
                    == self.matrix[line_index][index + distance]
            }
            Symmetry::Horizontal(index) => {
                self.matrix[index - distance - 1][line_index]
                    == self.matrix[index + distance][line_index]
            }
        }
    }

    fn possible_symmetries_vertical(&self) -> Vec<Symmetry> {
        (1..self.length_horizontal())
            .map(Symmetry::Vertical)
            .collect()
    }

    fn possible_symmetries_horizontal(&self) -> Vec<Symmetry> {
        (1..self.length_vertical())
            .map(Symmetry::Horizontal)
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
    Vertical(usize),
    Horizontal(usize),
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
        assert_eq!(result, None);
    }
}
