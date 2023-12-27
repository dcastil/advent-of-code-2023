use std::collections::HashSet;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<usize> {
    let contraption = Contraption::from_input(input);

    Some(contraption.tiles_visited(Path::new(0, 0, Direction::Right)))
}

/**
 * Would be much more efficient to store how many tiles will be visited when starting at each coordinate I visited previously and cache those values between calls to `contraption.tiles_visited`.
 * The current implementation is fast enough and didn't require any significant code changes from part one, so I didn't bother.
 */
pub fn part_two(input: &str) -> Option<usize> {
    let contraption = Contraption::from_input(input);
    // Assuming all lines have the same length
    let x_len = contraption.grid[0].len();
    let y_len = contraption.grid.len();

    let mut max_tiles_visited = 0;

    for y in 0..y_len {
        let tiles_visited_from_left = contraption.tiles_visited(Path::new(0, y, Direction::Right));
        let tiles_visited_from_right =
            contraption.tiles_visited(Path::new(x_len - 1, y, Direction::Left));

        if tiles_visited_from_left > max_tiles_visited {
            max_tiles_visited = tiles_visited_from_left;
        }

        if tiles_visited_from_right > max_tiles_visited {
            max_tiles_visited = tiles_visited_from_right;
        }
    }

    for x in 0..x_len {
        let tiles_visited_from_top = contraption.tiles_visited(Path::new(x, 0, Direction::Down));
        let tiles_visited_from_bottom =
            contraption.tiles_visited(Path::new(x, y_len - 1, Direction::Up));

        if tiles_visited_from_top > max_tiles_visited {
            max_tiles_visited = tiles_visited_from_top;
        }

        if tiles_visited_from_bottom > max_tiles_visited {
            max_tiles_visited = tiles_visited_from_bottom;
        }
    }

    Some(max_tiles_visited)
}

struct Contraption<'a> {
    grid: Vec<&'a [u8]>,
}

impl Contraption<'_> {
    fn from_input(input: &str) -> Contraption<'_> {
        Contraption {
            grid: input.lines().map(|line| line.as_bytes()).collect(),
        }
    }

    fn tiles_visited(&self, start_path: Path) -> usize {
        let mut visited_paths = HashSet::new();
        let mut paths = vec![start_path];

        while let Some(path) = paths.pop() {
            if visited_paths.contains(&path) {
                continue;
            }

            let char = self.grid[path.coordinate.y][path.coordinate.x];

            match char {
                b'.' => self.add_next_path(&path, path.entering_direction, &mut paths),
                b'/' => {
                    let next_direction = match path.entering_direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    };
                    self.add_next_path(&path, next_direction, &mut paths);
                }
                b'\\' => {
                    let next_direction = match path.entering_direction {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };
                    self.add_next_path(&path, next_direction, &mut paths);
                }
                b'|' => match path.entering_direction {
                    Direction::Left | Direction::Right => {
                        self.add_next_path(&path, Direction::Up, &mut paths);
                        self.add_next_path(&path, Direction::Down, &mut paths);
                    }
                    _ => self.add_next_path(&path, path.entering_direction, &mut paths),
                },
                b'-' => match path.entering_direction {
                    Direction::Up | Direction::Down => {
                        self.add_next_path(&path, Direction::Left, &mut paths);
                        self.add_next_path(&path, Direction::Right, &mut paths);
                    }
                    _ => self.add_next_path(&path, path.entering_direction, &mut paths),
                },
                _ => panic!("Unknown character {}", char as char),
            }

            visited_paths.insert(path);
        }

        visited_paths
            .into_iter()
            .map(|path| path.coordinate)
            .collect::<HashSet<_>>()
            .len()
    }

    fn add_next_path(&self, path: &Path, direction: Direction, paths: &mut Vec<Path>) {
        if self.is_in_bounds(&path.coordinate, direction) {
            paths.push(path.next(direction))
        }
    }

    fn is_in_bounds(&self, coordinate: &Coordinate, direction: Direction) -> bool {
        (direction == Direction::Up && coordinate.y != 0)
            || (direction == Direction::Left && coordinate.x != 0)
            || (direction == Direction::Down && coordinate.y != self.grid.len() - 1)
            || (direction == Direction::Right && coordinate.x != self.grid[coordinate.y].len() - 1)
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Path {
    coordinate: Coordinate,
    entering_direction: Direction,
}

impl Path {
    fn new(x: usize, y: usize, direction: Direction) -> Path {
        Path {
            coordinate: Coordinate { x, y },
            entering_direction: direction,
        }
    }

    fn next(&self, direction: Direction) -> Path {
        Path {
            coordinate: self.coordinate.next(direction),
            entering_direction: direction,
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn next(&self, direction: Direction) -> Coordinate {
        Coordinate {
            x: match direction {
                Direction::Left => self.x - 1,
                Direction::Right => self.x + 1,
                _ => self.x,
            },
            y: match direction {
                Direction::Up => self.y - 1,
                Direction::Down => self.y + 1,
                _ => self.y,
            },
        }
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
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
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
