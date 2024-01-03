use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<usize> {
    let graph = Graph::from_input(input);

    graph.smallest_path_cost_across()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

struct Graph {
    grid: Vec<Vec<usize>>,
}

impl Graph {
    fn from_input(input: &str) -> Graph {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

        Graph { grid }
    }

    fn smallest_path_cost_across(&self) -> Option<usize> {
        let start = Coordinate { x: 0, y: 0 };
        let end = Coordinate {
            x: self.grid[self.grid.len() - 1].len() - 1,
            y: self.grid.len() - 1,
        };

        self.smallest_path_cost(start, end)
    }

    /**
     * Uses A Start search algorithm: https://en.wikipedia.org/wiki/A*_search_algorithm
     */
    fn smallest_path_cost(&self, start: Coordinate, end: Coordinate) -> Option<usize> {
        let mut min_costs = HashMap::new();
        let mut nodes_to_visit = BinaryHeap::new();
        let mut position_origins = HashMap::new();

        let start_position = Position {
            coordinate: start.clone(),
            enter_direction: Direction::None,
            same_direction_count: 0,
        };

        min_costs.insert(start_position.clone(), 0);
        nodes_to_visit.push(Node {
            cost: 0,
            estimated_cost: self.estimate_cost_between(&start, &end),
            position: start_position,
        });

        while let Some(node) = nodes_to_visit.pop() {
            // We might have nodes with duplicate positions in the heap, so we need to check whether this node has non-minimal cost
            if node.cost > *min_costs.get(&node.position).unwrap_or(&usize::MAX) {
                continue;
            }

            if node.position.coordinate == end {
                self.print_path(&node, &position_origins);

                return Some(node.cost);
            }

            for neighbor in self.neighbors(&node, &end) {
                if neighbor.cost < *min_costs.get(&neighbor.position).unwrap_or(&usize::MAX) {
                    position_origins.insert(neighbor.position.clone(), node.position.clone());
                    min_costs.insert(neighbor.position.clone(), neighbor.cost);
                    nodes_to_visit.push(neighbor);
                }
            }
        }

        None
    }

    fn estimate_cost_between(&self, start: &Coordinate, end: &Coordinate) -> usize {
        let x_diff = if start.x > end.x {
            start.x - end.x
        } else {
            end.x - start.x
        };

        let y_diff = if start.y > end.y {
            start.y - end.y
        } else {
            end.y - start.y
        };

        x_diff + y_diff
    }

    fn neighbors<'a>(
        &'a self,
        node: &'a Node,
        end: &'a Coordinate,
    ) -> impl Iterator<Item = Node> + '_ {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
        .filter_map(move |direction| {
            if self.has_neighbor(node, direction) {
                let position = node.position.next(direction);
                let cost = node.cost + self.grid[position.coordinate.y][position.coordinate.x];

                Some(Node {
                    cost,
                    estimated_cost: cost + self.estimate_cost_between(&position.coordinate, end),
                    position,
                })
            } else {
                None
            }
        })
    }

    fn has_neighbor(&self, node: &Node, direction: Direction) -> bool {
        if node.position.enter_direction == direction && node.position.same_direction_count == 2
            || node.position.enter_direction == direction.opposite()
        {
            return false;
        }

        match direction {
            Direction::Up => node.position.coordinate.y != 0,
            Direction::Down => node.position.coordinate.y != self.grid.len() - 1,
            Direction::Left => node.position.coordinate.x != 0,
            Direction::Right => {
                node.position.coordinate.x != self.grid[node.position.coordinate.y].len() - 1
            }
            Direction::None => false,
        }
    }

    fn print_path(&self, node: &Node, position_origins: &HashMap<Position, Position>) {
        let mut path_coordinates = HashSet::new();

        let mut current = &node.position;

        while let Some(previous) = position_origins.get(current) {
            path_coordinates.insert(&current.coordinate);
            current = previous;
        }

        println!("Path taken:");

        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                if path_coordinates.contains(&Coordinate { x, y }) {
                    print!("\x1b[32m{}\x1b[0m", self.grid[y][x]);
                } else {
                    print!("{}", self.grid[y][x]);
                }
            }
            println!();
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
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

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Position {
    coordinate: Coordinate,
    enter_direction: Direction,
    same_direction_count: i8,
}

impl Position {
    fn next(&self, direction: Direction) -> Position {
        Position {
            coordinate: self.coordinate.next(direction),
            enter_direction: direction,
            same_direction_count: if self.enter_direction == direction {
                self.same_direction_count + 1
            } else {
                0
            },
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Node {
    cost: usize,
    estimated_cost: usize,
    position: Position,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .estimated_cost
            .cmp(&self.estimated_cost)
            .then_with(|| other.cost.cmp(&self.cost))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::None => Direction::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
