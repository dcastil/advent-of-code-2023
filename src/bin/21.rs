use std::collections::{HashMap, HashSet};

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<usize> {
    part_one_with_step_count(input, 65)
}

fn part_one_with_step_count(input: &str, step_count: u8) -> Option<usize> {
    let garden = Garden::from_input(input);
    let distance_map = garden.get_distance_map();
    let step_count_modulo = step_count % 2;

    Some(
        distance_map
            .values()
            .filter(|&&v| v <= step_count && v % 2 == step_count_modulo)
            .count(),
    )
}

// Solution from https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
pub fn part_two(input: &str) -> Option<usize> {
    let garden = Garden::from_input(input);
    let distance_map = garden.get_distance_map();

    let even_positions_corners_count = distance_map
        .values()
        .filter(|&&v| v % 2 == 0 && v > 65)
        .count();
    let odd_positions_corners_count = distance_map
        .values()
        .filter(|&&v| v % 2 == 1 && v > 65)
        .count();
    let even_positions_full_count = distance_map.values().filter(|&&v| v % 2 == 0).count();
    let odd_positions_full_count = distance_map.values().filter(|&&v| v % 2 == 1).count();

    // We know that the map has the same length in x and y direction
    let map_length = (garden.max_x + 1) as usize;
    let n = (26501365 - map_length / 2) / map_length;

    assert_eq!(n, 202300);

    let odd_tiles_count = (n + 1).pow(2);
    let even_tiles_count = n.pow(2);

    let positions_count = odd_tiles_count * odd_positions_full_count
        + even_tiles_count * even_positions_full_count
        - (n + 1) * odd_positions_corners_count
        + n * even_positions_corners_count;

    Some(positions_count)
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
                match character {
                    b'.' => (),
                    b'#' => {
                        obstacle_map.insert(Coordinate {
                            x: x as u8,
                            y: y as u8,
                        });
                    }
                    b'S' => {
                        start = Some(Coordinate {
                            x: x as u8,
                            y: y as u8,
                        });
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

    fn get_distance_map(&self) -> HashMap<Coordinate, u8> {
        let mut distance_map = HashMap::new();

        let mut edge_positions = vec![self.start.clone()];
        let mut distance = 0;

        while !edge_positions.is_empty() {
            let mut next_edge_positions = Vec::new();

            for coordinate in edge_positions {
                distance_map.entry(coordinate.clone()).or_insert_with(|| {
                    next_edge_positions.extend(
                        [
                            Direction::Up,
                            Direction::Down,
                            Direction::Left,
                            Direction::Right,
                        ]
                        .iter()
                        .filter_map(|direction| self.get_next_coordinate(&coordinate, direction)),
                    );

                    distance
                });
            }

            edge_positions = next_edge_positions;
            distance += 1;
        }

        distance_map
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
}
