use std::collections::HashSet;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let map_grid = Grid::new(input);

    let mut positions = map_grid.start_positions();
    let mut step_count = 0;

    while {
        for position in positions.iter_mut() {
            *position = map_grid.next_position(position);
        }

        step_count += 1;

        !are_positions_equal(&positions)
    } {}

    Some(step_count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map_grid = Grid::new(input);
    let mut visited_coordinates = HashSet::new();

    let mut current_position = map_grid.start_positions().into_iter().next().unwrap();
    let start_position = current_position.clone();
    let mut right_turns: i32 = 0;

    while {
        let next_position = map_grid.next_position_with_start(&current_position, &start_position);

        match current_position.direction.to_turn(&next_position.direction) {
            Turn::Right => right_turns += 1,
            Turn::Left => right_turns -= 1,
            _ => (),
        }

        visited_coordinates.insert(next_position.coordinate.clone());

        current_position = next_position;

        start_position.coordinate != current_position.coordinate
    } {}

    let loop_tiles_count = visited_coordinates.len();
    let is_clockwise = right_turns > 0;

    while {
        let next_position = map_grid.next_position_with_start(&current_position, &start_position);

        map_grid.visit_inside_tiles(
            &current_position,
            &next_position,
            is_clockwise,
            &mut visited_coordinates,
        );

        current_position = next_position;

        start_position.coordinate != current_position.coordinate
    } {}

    Some(visited_coordinates.len() - loop_tiles_count)
}

fn are_positions_equal(positions: &[Position]) -> bool {
    let coordinate = &positions[0].coordinate;

    positions
        .iter()
        .skip(1)
        .all(|position| &position.coordinate == coordinate)
}

struct Grid<'a> {
    grid: Vec<&'a str>,
}

impl Grid<'_> {
    fn new(input: &str) -> Grid {
        Grid {
            grid: input.lines().collect(),
        }
    }

    fn start_positions(&self) -> Vec<Position> {
        let coordinate = self.start_coordinate();

        let mut start_positions = Vec::new();

        let position_up = Position::new(coordinate.clone(), Direction::Up);
        if self.is_connected(&position_up) {
            start_positions.push(position_up);
        }

        let position_down = Position::new(coordinate.clone(), Direction::Down);
        if self.is_connected(&position_down) {
            start_positions.push(position_down);
        }

        let position_left = Position::new(coordinate.clone(), Direction::Left);
        if self.is_connected(&position_left) {
            start_positions.push(position_left);
        }

        let position_right = Position::new(coordinate, Direction::Right);
        if self.is_connected(&position_right) {
            start_positions.push(position_right);
        }

        start_positions
    }

    fn start_coordinate(&self) -> Coordinate {
        for (y, line) in self.grid.iter().enumerate() {
            for (x, &character) in line.as_bytes().iter().enumerate() {
                if character == b'S' {
                    return Coordinate::new(x, y);
                }
            }
        }

        panic!("No starting position found")
    }

    fn is_connected(&self, position: &Position) -> bool {
        if !self.is_in_bounds(position) {
            return false;
        }

        let next_coordinate = position.next_coordinate_unchecked();

        position
            .direction
            .is_connected(*self.char_unchecked(&next_coordinate))
    }

    fn is_in_bounds(&self, position: &Position) -> bool {
        (position.direction == Direction::Up && position.coordinate.y != 0)
            || (position.direction == Direction::Left && position.coordinate.x != 0)
            || (position.direction == Direction::Down
                && position.coordinate.y != self.grid.len() - 1)
            || (position.direction == Direction::Right
                && position.coordinate.x != self.grid[position.coordinate.y].len() - 1)
    }

    fn next_position(&self, position: &Position) -> Position {
        let next_coordinate = position.next_coordinate_unchecked();
        let next_direction = position
            .direction
            .next(*self.char_unchecked(&next_coordinate))
            .unwrap();

        Position::new(next_coordinate, next_direction)
    }

    fn char_unchecked(&self, coordinate: &Coordinate) -> &u8 {
        &self.grid[coordinate.y].as_bytes()[coordinate.x]
    }

    fn next_position_with_start(&self, position: &Position, start_position: &Position) -> Position {
        let next_coordinate = position.next_coordinate_unchecked();
        let next_character = *self.char_unchecked(&next_coordinate);

        if next_character == b'S' {
            start_position.clone()
        } else {
            let next_direction = position
                .direction
                .next(*self.char_unchecked(&next_coordinate))
                .unwrap();

            Position::new(next_coordinate, next_direction)
        }
    }

    fn visit_inside_tiles(
        &self,
        current_position: &Position,
        next_position: &Position,
        is_clockwise: bool,
        visited_coordinates: &mut HashSet<Coordinate>,
    ) {
        let inside_directions = current_position
            .direction
            .to_inside_directions(&next_position.direction, is_clockwise);

        for direction in inside_directions {
            let position = next_position.with_direction(direction);

            self.visit_tiles_recursively(&position, visited_coordinates);
        }
    }

    fn visit_tiles_recursively(
        &self,
        position: &Position,
        visited_coordinates: &mut HashSet<Coordinate>,
    ) {
        if !self.is_in_bounds(position) {
            return;
        }

        let coordinate = position.next_coordinate_unchecked();
        let previous_direction = position.direction.inverse();

        if visited_coordinates.insert(coordinate) {
            for direction in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                if direction != previous_direction {
                    self.visit_tiles_recursively(
                        &Position::new(position.next_coordinate_unchecked(), direction),
                        visited_coordinates,
                    )
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Position {
    coordinate: Coordinate,
    direction: Direction,
}

impl Position {
    fn new(coordinate: Coordinate, direction: Direction) -> Position {
        Position {
            coordinate,
            direction,
        }
    }

    fn with_direction(&self, direction: Direction) -> Position {
        Position {
            coordinate: self.coordinate.clone(),
            direction,
        }
    }

    fn next_coordinate_unchecked(&self) -> Coordinate {
        Coordinate::new(
            match self.direction {
                Direction::Left => self.coordinate.x - 1,
                Direction::Right => self.coordinate.x + 1,
                _ => self.coordinate.x,
            },
            match self.direction {
                Direction::Up => self.coordinate.y - 1,
                Direction::Down => self.coordinate.y + 1,
                _ => self.coordinate.y,
            },
        )
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_pipe_char(character: u8) -> [Direction; 2] {
        match character {
            b'|' => [Direction::Up, Direction::Down],
            b'-' => [Direction::Left, Direction::Right],
            b'L' => [Direction::Up, Direction::Right],
            b'J' => [Direction::Up, Direction::Left],
            b'7' => [Direction::Down, Direction::Left],
            b'F' => [Direction::Down, Direction::Right],
            _ => panic!("Invalid pipe character {}", character as char),
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

    fn is_connected(&self, character: u8) -> bool {
        if character == b'.' {
            false
        } else {
            Direction::from_pipe_char(character).contains(&self.inverse())
        }
    }

    fn next(&self, character: u8) -> Option<Direction> {
        let inverse_direction = self.inverse();

        Direction::from_pipe_char(character)
            .into_iter()
            .find(|direction| *direction != inverse_direction)
    }

    fn inverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn to_turn(&self, next_direction: &Direction) -> Turn {
        match (4 + self.turn_value() - next_direction.turn_value()) % 4 {
            0 => Turn::None,
            1 => Turn::Left,
            3 => Turn::Right,
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

    fn to_inside_directions(
        &self,
        next_direction: &Direction,
        is_clockwise: bool,
    ) -> Vec<Direction> {
        let mut directions = Vec::new();

        let mut current_turn_value = if is_clockwise {
            next_direction.turn_value()
        } else {
            self.inverse().turn_value()
        };

        let end_turn_value = if is_clockwise {
            self.inverse().turn_value()
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
}

enum Turn {
    Left,
    Right,
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10));
    }
}
