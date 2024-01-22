use std::collections::HashSet;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<usize> {
    part_one_with_step_count(input, 64)
}

fn part_one_with_step_count(input: &str, step_count: u8) -> Option<usize> {
    let garden = Garden::from_input(input);

    Some(garden.get_possible_positions(step_count).len())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

struct Garden {
    obstacle_map: HashSet<Coordinate>,
    start: Coordinate,
    max_x: u8,
    max_y: u8,
}

impl Garden {
    fn from_input(input: &str) -> Garden {
        let mut obstacle_map = HashSet::new();
        let mut start = None;
        let mut max_x = 0;
        let mut max_y = 0;

        for (y, line) in input.lines().enumerate() {
            for (x, character) in line.as_bytes().iter().enumerate() {
                let coordinate = Coordinate {
                    x: x as u8,
                    y: y as u8,
                };

                match character {
                    b'#' => {
                        obstacle_map.insert(coordinate);
                    }
                    b'.' => (),
                    b'S' => {
                        start = Some(coordinate);
                    }
                    _ => panic!("Unknown character: {}", character),
                }

                if x > max_x {
                    max_x = x;
                }
            }

            max_y = y;
        }

        Garden {
            obstacle_map,
            start: start.unwrap(),
            max_x: max_x as u8,
            max_y: max_y as u8,
        }
    }

    fn get_possible_positions(&self, step_count: u8) -> HashSet<Coordinate> {
        let mut positions = HashSet::from([self.start.clone()]);

        for _ in 0..step_count {
            let mut new_positions = HashSet::new();

            for coordinate in positions {
                new_positions.extend(
                    [
                        Direction::Up,
                        Direction::Down,
                        Direction::Left,
                        Direction::Right,
                    ]
                    .iter()
                    .filter_map(|direction| self.get_next_coordinate(&coordinate, direction)),
                );
            }

            positions = new_positions;
        }

        positions
    }

    fn get_next_coordinate(
        &self,
        coordiante: &Coordinate,
        direction: &Direction,
    ) -> Option<Coordinate> {
        if self.is_in_bounds(coordiante, direction) {
            let next_coordinate = coordiante.next(direction);

            if !self.obstacle_map.contains(&next_coordinate) {
                return Some(next_coordinate);
            }
        }

        None
    }

    fn is_in_bounds(&self, coordinate: &Coordinate, direction: &Direction) -> bool {
        match direction {
            Direction::Up => coordinate.y != 0,
            Direction::Left => coordinate.x != 0,
            Direction::Down => coordinate.y != self.max_y,
            Direction::Right => coordinate.x != self.max_x,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Coordinate {
    x: u8,
    y: u8,
}

impl Coordinate {
    fn next(&self, direction: &Direction) -> Coordinate {
        match direction {
            Direction::Up => Coordinate {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Coordinate {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Coordinate {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Coordinate {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
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
        let result =
            part_one_with_step_count(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
