advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let map_grid = Grid::new(input);

    let mut positions = map_grid.get_start_positions();
    let mut step_count = 0;

    while step_count == 0 || !are_positions_equal(&positions) {
        for position in positions.iter_mut() {
            *position = map_grid.get_next_position(position);
        }

        step_count += 1;
    }

    Some(step_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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

    fn get_start_positions(&self) -> Vec<Position> {
        let coordinate = self.get_start_coordinate();

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

    fn get_start_coordinate(&self) -> Coordinate {
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
        if position.is_unsafe() {
            return false;
        }

        let next_coordinate = position.get_next_coordinate_unchecked();

        self.get_char(next_coordinate).map_or(false, |&character| {
            position.direction.is_connected(character)
        })
    }

    fn get_next_position(&self, position: &Position) -> Position {
        let next_coordinate = position.get_next_coordinate_unchecked();
        let next_direction = position
            .direction
            .get_next(*self.get_char_unchecked(&next_coordinate))
            .unwrap();

        Position::new(next_coordinate, next_direction)
    }

    fn get_char(&self, coordinate: Coordinate) -> Option<&u8> {
        self.grid
            .get(coordinate.y)
            .and_then(|line| line.as_bytes().get(coordinate.x))
    }

    fn get_char_unchecked(&self, coordinate: &Coordinate) -> &u8 {
        &self.grid[coordinate.y].as_bytes()[coordinate.x]
    }
}

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

    fn get_next_coordinate_unchecked(&self) -> Coordinate {
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

    fn is_unsafe(&self) -> bool {
        (self.coordinate.x == 0 && self.direction == Direction::Left)
            || (self.coordinate.y == 0 && self.direction == Direction::Up)
    }
}

#[derive(PartialEq, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
    }
}

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_from_pipe_char(character: u8) -> [Direction; 2] {
        match character {
            b'|' => [Direction::Up, Direction::Down],
            b'-' => [Direction::Left, Direction::Right],
            b'L' => [Direction::Up, Direction::Right],
            b'J' => [Direction::Up, Direction::Left],
            b'7' => [Direction::Down, Direction::Left],
            b'F' => [Direction::Down, Direction::Right],
            _ => panic!("Invalid pipe character"),
        }
    }

    fn is_connected(&self, character: u8) -> bool {
        if character == b'.' {
            false
        } else {
            Direction::get_from_pipe_char(character).contains(&self.get_inverse())
        }
    }

    fn get_next(&self, character: u8) -> Option<Direction> {
        let inverse_direction = self.get_inverse();

        Direction::get_from_pipe_char(character)
            .into_iter()
            .find(|direction| *direction != inverse_direction)
    }

    fn get_inverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
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
        assert_eq!(result, None);
    }
}
