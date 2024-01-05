use std::collections::HashSet;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<usize> {
    let instructions: Vec<_> = input.lines().map(Instruction::from_line_1).collect();

    let mut visited_coordinates = HashSet::new();

    let mut current_coordinate = Coordinate { x: 0, y: 0 };
    let mut current_direction = instructions[0].direction;
    let mut right_turns = 0;

    for instruction in instructions.iter() {
        right_turns += current_direction.to_right_turn(instruction.direction);
        current_direction = instruction.direction;

        for _ in 1..=instruction.distance {
            current_coordinate = current_coordinate.next(current_direction, 1);
            visited_coordinates.insert(current_coordinate.clone());
        }
    }

    let is_clockwise = right_turns > 0;

    for instruction in instructions {
        current_coordinate = current_coordinate.next(instruction.direction, 1);
        visit_inside_tiles(
            current_direction,
            instruction.direction,
            &current_coordinate,
            is_clockwise,
            &mut visited_coordinates,
        );

        current_direction = instruction.direction;

        for _ in 2..=instruction.distance {
            current_coordinate = current_coordinate.next(current_direction, 1);
            visit_inside_tiles(
                current_direction,
                current_direction,
                &current_coordinate,
                is_clockwise,
                &mut visited_coordinates,
            );
        }
    }

    Some(visited_coordinates.len())
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut current_coordinate = Coordinate { x: 0, y: 0 };
    let mut area_twice = 0;

    for instruction in input.lines().map(Instruction::from_line_2) {
        let next_coordinate = current_coordinate.next(instruction.direction, instruction.distance);

        // Shoelace formula: https://en.wikipedia.org/wiki/Shoelace_formula
        area_twice += current_coordinate.x * next_coordinate.y
            - next_coordinate.x * current_coordinate.y
            + instruction.distance;

        current_coordinate = next_coordinate;
    }

    Some(area_twice.abs() / 2 + 1)
}

fn visit_inside_tiles(
    previous_direction: Direction,
    current_direction: Direction,
    current_coordinate: &Coordinate,
    is_clockwise: bool,
    visited_coordinates: &mut HashSet<Coordinate>,
) {
    let inside_directions =
        previous_direction.to_inside_directions(current_direction, is_clockwise);

    for direction in inside_directions {
        visit_tiles_recursively(current_coordinate, direction, visited_coordinates);
    }
}

fn visit_tiles_recursively(
    previous_coordinate: &Coordinate,
    direction: Direction,
    visited_coordinates: &mut HashSet<Coordinate>,
) {
    let coordinate = previous_coordinate.next(direction, 1);

    if visited_coordinates.insert(coordinate) {
        let direction_back = direction.reverse();
        let coordinate = previous_coordinate.next(direction, 1);

        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if direction != direction_back {
                visit_tiles_recursively(&coordinate, direction, visited_coordinates)
            }
        }
    }
}

struct Instruction {
    direction: Direction,
    distance: i64,
}

impl Instruction {
    fn from_line_1(line: &str) -> Instruction {
        let mut split_iterator = line.split(' ');

        Instruction {
            direction: Direction::from_string_1(split_iterator.next().unwrap()),
            distance: split_iterator.next().unwrap().parse().unwrap(),
        }
    }

    fn from_line_2(line: &str) -> Instruction {
        let string = line.split(' ').nth(2).unwrap();

        Instruction {
            direction: Direction::from_string_2(&string[7..8]),
            distance: i64::from_str_radix(&string[2..7], 16).unwrap(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_string_1(string: &str) -> Direction {
        match string {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }

    fn from_string_2(string: &str) -> Direction {
        match string {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => panic!("Invalid direction"),
        }
    }

    fn from_turn_value(turn_value: u8) -> Direction {
        match turn_value {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!("Invalid turn value {}", turn_value),
        }
    }

    fn to_right_turn(self, next_direction: Direction) -> i8 {
        match (4 + self.turn_value() - next_direction.turn_value()) % 4 {
            0 => 0,
            1 => -1,
            3 => 1,
            _ => panic!("Invalid turn from {:?} to {:?}", self, next_direction),
        }
    }

    fn turn_value(&self) -> u8 {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }

    fn to_inside_directions(self, next_direction: Direction, is_clockwise: bool) -> Vec<Direction> {
        let mut directions = Vec::new();

        let mut current_turn_value = if is_clockwise {
            next_direction.turn_value()
        } else {
            self.reverse().turn_value()
        };

        let end_turn_value = if is_clockwise {
            self.reverse().turn_value()
        } else {
            next_direction.turn_value()
        };

        while {
            current_turn_value = (current_turn_value + 1) % 4;

            current_turn_value != end_turn_value
        } {
            directions.push(Direction::from_turn_value(current_turn_value));
        }

        directions
    }

    fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Coordinate {
    x: i64,
    y: i64,
}

impl Coordinate {
    fn next(&self, direction: Direction, distance: i64) -> Coordinate {
        Coordinate {
            x: match direction {
                Direction::Left => self.x - distance,
                Direction::Right => self.x + distance,
                _ => self.x,
            },
            y: match direction {
                Direction::Up => self.y - distance,
                Direction::Down => self.y + distance,
                _ => self.y,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952_408_144_115));
    }
}
