use std::collections::HashSet;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let grid = GalaxyGrid::from_input(input);

    let mut processed_galaxies = Vec::new();
    let mut distances_sum = 0;

    for galaxy in grid.galaxies() {
        for processed_galaxy in processed_galaxies.iter() {
            distances_sum += galaxy.distance(processed_galaxy);
        }

        processed_galaxies.push(galaxy);
    }

    Some(distances_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

struct GalaxyGrid<'a> {
    grid: Vec<&'a str>,
}

impl GalaxyGrid<'_> {
    fn from_input(input: &str) -> GalaxyGrid {
        GalaxyGrid {
            grid: input.lines().collect(),
        }
    }

    fn galaxies(&self) -> Vec<Galaxy> {
        let mut galaxies = Vec::new();
        let xs_without_galaxies: HashSet<_> = self.xs_without_galaxies().collect();

        let mut current_y = 0;

        for line in self.grid.iter() {
            let mut current_x = 0;
            let mut y_has_galaxies = false;

            for (x_index, character) in line.as_bytes().iter().enumerate() {
                if xs_without_galaxies.contains(&x_index) {
                    current_x += 2;
                    continue;
                }

                if *character == b'#' {
                    galaxies.push(Galaxy::new(current_x, current_y));

                    y_has_galaxies = true;
                }

                current_x += 1;
            }

            current_y += if y_has_galaxies { 1 } else { 2 };
        }

        galaxies
    }

    fn xs_without_galaxies(&self) -> impl Iterator<Item = usize> + '_ {
        // Assuming every line has the same length
        let x_count = self.grid[0].len();

        (0..x_count).filter(|&y| self.grid.iter().all(|line| line.as_bytes()[y] == b'.'))
    }
}

struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    fn new(x: usize, y: usize) -> Galaxy {
        Galaxy { x, y }
    }

    fn distance(&self, other: &Galaxy) -> usize {
        let distance_x = if self.x < other.x {
            other.x - self.x
        } else {
            self.x - other.x
        };

        let distance_y = if self.y < other.y {
            other.y - self.y
        } else {
            self.y - other.y
        };

        distance_x + distance_y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
