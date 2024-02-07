use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    hash::Hash,
    rc::{Rc, Weak},
};

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u16> {
    let brick_stack = BrickStack::from_input(input);

    Some(brick_stack.redundant_bricks_count())
}

pub fn part_two(input: &str) -> Option<u32> {
    let brick_stack = BrickStack::from_input(input);

    Some(brick_stack.dependants_count())
}

struct BrickStack {
    bricks: Vec<Rc<Brick>>,
}

impl BrickStack {
    fn from_input(input: &str) -> BrickStack {
        let mut max_occupied_zs: HashMap<FlatPosition, (i16, Rc<Brick>)> = HashMap::new();

        let mut bricks = input
            .lines()
            .map(|line| Rc::new(Brick::from_line(line)))
            .collect::<Vec<_>>();

        bricks.sort_by_key(|brick| brick.min_z());

        for brick in &mut bricks {
            let mut min_z = 1;
            let mut supported_by = Vec::new();

            for position in brick.walk_x_y() {
                if let Some((occupied_z, brick)) = max_occupied_zs.get(&position) {
                    if *occupied_z >= min_z {
                        min_z = occupied_z + 1;
                        supported_by = vec![Rc::clone(brick)];
                    } else if *occupied_z == min_z - 1 && !supported_by.contains(brick) {
                        supported_by.push(Rc::clone(brick));
                    }
                }
            }

            *brick = Rc::new(brick.to_stationary(min_z, supported_by));

            let brick_max_z = brick.max_z();

            for position in brick.walk_x_y() {
                max_occupied_zs.insert(position, (brick_max_z, Rc::clone(brick)));
            }
        }

        for brick_top in &bricks {
            for brick in &brick_top.supported_by {
                brick.supports.borrow_mut().push(Rc::downgrade(brick_top));
            }
        }

        BrickStack { bricks }
    }

    fn redundant_bricks_count(&self) -> u16 {
        self.bricks
            .iter()
            .filter(|brick| brick.is_redundant())
            .count() as u16
    }

    fn dependants_count(&self) -> u32 {
        self.bricks
            .iter()
            .map(|brick| {
                let mut dependants =
                    HashSet::from_iter(brick.supported_by.iter().map(|brick| brick.hashable()));

                Self::dependants_count_recursive(brick, &mut dependants) - 1
            })
            .sum()
    }

    fn dependants_count_recursive(
        brick: &Rc<Brick>,
        dependants: &mut HashSet<HashableBrick>,
    ) -> u32 {
        if brick
            .supported_by
            .iter()
            .any(|brick| !dependants.contains(&brick.hashable()))
        {
            return 0;
        }

        dependants.insert(brick.hashable());

        let count = brick
            .supports
            .borrow()
            .iter()
            .map(|brick| Self::dependants_count_recursive(&brick.upgrade().unwrap(), dependants))
            .sum::<u32>()
            + 1;

        count
    }
}

struct Brick {
    start: Position,
    end: Position,
    supported_by: Vec<Rc<Brick>>,
    supports: RefCell<Vec<Weak<Brick>>>,
}

impl Brick {
    fn from_line(line: &str) -> Brick {
        let mut parts_iterator = line.split('~');
        let mut start_iterator = parts_iterator.next().unwrap().split(',');
        let mut end_iterator = parts_iterator.next().unwrap().split(',');

        Brick {
            start: Position {
                x: start_iterator.next().unwrap().parse().unwrap(),
                y: start_iterator.next().unwrap().parse().unwrap(),
                z: start_iterator.next().unwrap().parse().unwrap(),
            },
            end: Position {
                x: end_iterator.next().unwrap().parse().unwrap(),
                y: end_iterator.next().unwrap().parse().unwrap(),
                z: end_iterator.next().unwrap().parse().unwrap(),
            },
            supported_by: Vec::new(),
            supports: RefCell::new(Vec::new()),
        }
    }

    fn to_stationary(&self, z: i16, supported_by: Vec<Rc<Brick>>) -> Brick {
        let z_difference = z - self.start.z.min(self.end.z);

        Brick {
            start: self.start.to_translated_z(z_difference),
            end: self.end.to_translated_z(z_difference),
            supported_by,
            supports: RefCell::new(Vec::new()),
        }
    }

    fn walk_x_y(&self) -> impl Iterator<Item = FlatPosition> + '_ {
        (self.start.x..=self.end.x)
            .flat_map(move |x| (self.start.y..=self.end.y).map(move |y| FlatPosition { x, y }))
    }

    fn min_z(&self) -> i16 {
        self.start.z.min(self.end.z)
    }

    fn max_z(&self) -> i16 {
        self.start.z.max(self.end.z)
    }

    fn is_redundant(&self) -> bool {
        self.supports
            .borrow()
            .iter()
            .all(|brick| brick.upgrade().unwrap().supported_by.len() > 1)
    }

    fn hashable(&self) -> HashableBrick {
        HashableBrick {
            start: self.start.clone(),
            end: self.end.clone(),
        }
    }
}

impl PartialEq for Brick {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct HashableBrick {
    start: Position,
    end: Position,
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Position {
    x: i16,
    y: i16,
    z: i16,
}

impl Position {
    fn to_translated_z(&self, z: i16) -> Position {
        Position {
            x: self.x,
            y: self.y,
            z: self.z + z,
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
struct FlatPosition {
    x: i16,
    y: i16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
