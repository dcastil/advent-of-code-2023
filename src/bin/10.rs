advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let map_grid = get_map_grid(input);

    let mut positions = get_starting_positions(&map_grid);
    let mut step_count = 0;

    while step_count == 0 || !are_positions_equal(&positions) {
        for position in positions.iter_mut() {
            *position = get_next_position(&map_grid, position);
        }

        step_count += 1;
    }

    Some(step_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn get_map_grid(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn get_starting_positions(grid: &[&str]) -> Vec<(usize, usize, Direction)> {
    let (x, y) = get_start_coordinates(grid);

    let mut start_positions = Vec::new();

    if is_connected(grid, (x, y, Direction::Up)) {
        start_positions.push((x, y, Direction::Up));
    }

    if is_connected(grid, (x, y, Direction::Down)) {
        start_positions.push((x, y, Direction::Down));
    }

    if is_connected(grid, (x, y, Direction::Left)) {
        start_positions.push((x, y, Direction::Left));
    }

    if is_connected(grid, (x, y, Direction::Right)) {
        start_positions.push((x, y, Direction::Right));
    }

    start_positions
}

fn get_start_coordinates(grid: &[&str]) -> (usize, usize) {
    for (y, line) in grid.iter().enumerate() {
        for (x, &character) in line.as_bytes().iter().enumerate() {
            if character == b'S' {
                return (x, y);
            }
        }
    }

    panic!("No starting position found")
}

fn is_connected(grid: &[&str], (x, y, direction): (usize, usize, Direction)) -> bool {
    if (x == 0 && direction == Direction::Left) || (y == 0 && direction == Direction::Up) {
        return false;
    }

    let next_x = match direction {
        Direction::Left => x - 1,
        Direction::Right => x + 1,
        _ => x,
    };

    let next_y = match direction {
        Direction::Up => y - 1,
        Direction::Down => y + 1,
        _ => y,
    };

    let next_character = grid
        .get(next_y)
        .and_then(|line| line.as_bytes().get(next_x));

    if let Some(&character) = next_character {
        if character == b'.' {
            false
        } else {
            get_pipe_directions(character).contains(&get_inverse_direction(&direction))
        }
    } else {
        false
    }
}

fn get_next_position(
    grid: &[&str],
    (x, y, direction): &(usize, usize, Direction),
) -> (usize, usize, Direction) {
    let next_x = match direction {
        Direction::Left => x - 1,
        Direction::Right => x + 1,
        _ => *x,
    };

    let next_y = match direction {
        Direction::Up => y - 1,
        Direction::Down => y + 1,
        _ => *y,
    };

    let inverse_direction = get_inverse_direction(direction);
    let [direction_a, direction_b] = get_pipe_directions(grid[next_y].as_bytes()[next_x]);

    let next_direction = if direction_a == inverse_direction {
        direction_b
    } else {
        direction_a
    };

    (next_x, next_y, next_direction)
}

fn are_positions_equal<T>(positions: &[(usize, usize, T)]) -> bool {
    let (x, y, _) = positions[0];

    positions
        .iter()
        .skip(1)
        .all(|(inner_x, inner_y, _)| x == *inner_x && y == *inner_y)
}

fn get_pipe_directions(character: u8) -> [Direction; 2] {
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

fn get_inverse_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}

#[derive(PartialEq, Debug)]
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
