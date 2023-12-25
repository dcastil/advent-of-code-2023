use std::hash::{Hash, Hasher};
use std::{collections::HashMap, vec};

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

pub fn part_two(input: &str) -> Option<usize> {
    let mut platform = Platform::from_input(input);
    let mut cycle_map = HashMap::new();

    for index in 0..1000 {
        platform.tilt_cycle();

        if let Some(previous_index) = cycle_map.get(&platform) {
            let cycle_length = index - previous_index;

            let remaining_cycles = (1_000_000_000 - index - 1) % cycle_length;

            for _ in 0..remaining_cycles {
                platform.tilt_cycle();
            }

            return Some(platform.count_load());
        } else {
            cycle_map.insert(platform.clone(), index);
        }
    }

    None
}

#[derive(PartialEq, Eq, Clone)]
struct Platform {
    grid: HashMap<Coordinate, Element>,
    len_x: usize,
    len_y: usize,
}

impl Platform {
    fn from_input(input: &str) -> Platform {
        let mut grid = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;

        for (y, line) in input.lines().enumerate() {
            max_y = y;

            for (x, character) in line.chars().enumerate() {
                if x > max_x {
                    max_x = x;
                }

                if character == '#' {
                    grid.insert(Coordinate { x, y }, Element::Fixed);
                } else if character == 'O' {
                    grid.insert(Coordinate { x, y }, Element::Movable);
                }
            }
        }

        Platform {
            grid,
            len_x: max_x + 1,
            len_y: max_y + 1,
        }
    }

    fn tilt_cycle(&mut self) {
        self.tilt(Direction::Up);
        self.tilt(Direction::Left);
        self.tilt(Direction::Down);
        self.tilt(Direction::Right);
    }

    fn tilt(&mut self, direction: Direction) {
        let mut next_empty_slots = match direction {
            Direction::Up => vec![0; self.len_x],
            Direction::Down => vec![self.len_y - 1; self.len_x],
            Direction::Left => vec![0; self.len_y],
            Direction::Right => vec![self.len_x - 1; self.len_y],
        };

        match direction {
            Direction::Up => {
                for y in 0..self.len_y {
                    for x in 0..self.len_x {
                        self.handle_position(
                            Coordinate { x, y },
                            &direction,
                            &mut next_empty_slots,
                        );
                    }
                }
            }
            Direction::Down => {
                for y in (0..self.len_y).rev() {
                    for x in 0..self.len_x {
                        self.handle_position(
                            Coordinate { x, y },
                            &direction,
                            &mut next_empty_slots,
                        );
                    }
                }
            }
            Direction::Left => {
                for x in 0..self.len_x {
                    for y in 0..self.len_y {
                        self.handle_position(
                            Coordinate { x, y },
                            &direction,
                            &mut next_empty_slots,
                        );
                    }
                }
            }
            Direction::Right => {
                for x in (0..self.len_x).rev() {
                    for y in 0..self.len_y {
                        self.handle_position(
                            Coordinate { x, y },
                            &direction,
                            &mut next_empty_slots,
                        );
                    }
                }
            }
        }
    }

    fn handle_position(
        &mut self,
        coordinate: Coordinate,
        direction: &Direction,
        next_empty_slots: &mut [usize],
    ) {
        if let Some(element) = self.grid.get(&coordinate) {
            let (outer_index, inner_index) = match direction {
                Direction::Up | Direction::Down => (coordinate.y, coordinate.x),
                Direction::Left | Direction::Right => (coordinate.x, coordinate.y),
            };

            match element {
                Element::Fixed => {
                    next_empty_slots[inner_index] = match direction {
                        Direction::Up | Direction::Left => outer_index + 1,
                        Direction::Down | Direction::Right => {
                            if outer_index == 0 {
                                0
                            } else {
                                outer_index - 1
                            }
                        }
                    };
                }
                Element::Movable => {
                    let moved_coordinate = match direction {
                        Direction::Up | Direction::Down => Coordinate {
                            x: coordinate.x,
                            y: next_empty_slots[inner_index],
                        },
                        Direction::Left | Direction::Right => Coordinate {
                            x: next_empty_slots[inner_index],
                            y: coordinate.y,
                        },
                    };

                    self.grid.remove(&coordinate);
                    self.grid.insert(moved_coordinate, Element::Movable);

                    next_empty_slots[inner_index] = match direction {
                        Direction::Up | Direction::Left => next_empty_slots[inner_index] + 1,
                        Direction::Down | Direction::Right => {
                            if next_empty_slots[inner_index] == 0 {
                                0
                            } else {
                                next_empty_slots[inner_index] - 1
                            }
                        }
                    };
                }
            }
        }
    }

    fn count_load(&self) -> usize {
        self.grid
            .iter()
            .filter_map(|(coordinate, element)| match element {
                Element::Movable => Some(self.len_y - coordinate.y),
                Element::Fixed => None,
            })
            .sum()
    }
}

impl Hash for Platform {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for y in 0..self.len_y {
            for x in 0..self.len_x {
                let coordinate = Coordinate { x, y };

                if let Some(element) = self.grid.get(&coordinate) {
                    coordinate.hash(state);
                    element.hash(state)
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Element {
    Movable,
    Fixed,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
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
        assert_eq!(result, Some(64));
    }
}
