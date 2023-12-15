use core::panic;
use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let navigation_instructions = lines.next().unwrap();
    let nodes_map = get_nodes_hash_map(lines.skip(1));

    let (steps_count, _) =
        get_steps_until_end(navigation_instructions, &nodes_map, "AAA", |node| {
            node == "ZZZ"
        });

    Some(steps_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let navigation_instructions = lines.next().unwrap();
    let nodes_map = get_nodes_hash_map(lines.clone().skip(1));

    let start_nodes = get_starting_nodes(lines.skip(1));
    let end_nodes_per_start_node = start_nodes
        .iter()
        .map(|start_node| {
            let mut end_nodes = Vec::new();
            let mut current_node_id = *start_node;

            for _ in 0..start_nodes.len() {
                let result = get_steps_until_end(
                    navigation_instructions,
                    &nodes_map,
                    current_node_id,
                    |node| node.ends_with('Z'),
                );

                end_nodes.push(result);

                current_node_id = result.1;
            }

            end_nodes
        })
        .collect::<Vec<_>>();

    //  Based on input I have I know that this is true and therefore skip more complicated computations.
    if end_nodes_per_start_node.iter().all(|end_nodes| {
        let mut step_count_outer = 0;

        end_nodes
            .iter()
            .enumerate()
            .all(|(index, (step_count, _))| {
                if index == 0 {
                    step_count_outer = *step_count
                }

                *step_count == step_count_outer
            })
    }) {
        let mut factor_map = HashMap::new();

        for end_nodes in end_nodes_per_start_node {
            let factors = get_prime_factorization(end_nodes[0].0);

            for (factor, count) in factors {
                let existing_count = factor_map.get(&factor).unwrap_or(&0);

                if count > *existing_count {
                    factor_map.insert(factor, count);
                }
            }
        }

        let mut result = 1;

        for (factor, count) in factor_map {
            result *= factor.pow(count);
        }

        Some(result)
    } else {
        None
    }
}

fn get_nodes_hash_map<'a>(lines: impl Iterator<Item = &'a str>) -> HashMap<&'a str, Node<'a>> {
    let mut map = HashMap::new();

    for line in lines {
        map.insert(&line[0..3], Node::new(&line[7..10], &line[12..15]));
    }

    map
}

fn get_steps_until_end<'a>(
    navigation_instructions: &'a str,
    nodes_map: &'a HashMap<&'a str, Node<'a>>,
    start: &'a str,
    is_end: fn(&str) -> bool,
) -> (u64, &'a str) {
    let mut current_node_id = start;
    let mut step_count = 0;

    while step_count == 0 || !is_end(current_node_id) {
        for char in navigation_instructions.chars() {
            step_count += 1;

            let current_node = nodes_map.get(current_node_id).unwrap();

            current_node_id = match char {
                'L' => current_node.left,
                'R' => current_node.right,
                _ => panic!("Invalid navigation instruction"),
            };

            if is_end(current_node_id) {
                break;
            }
        }
    }

    (step_count, current_node_id)
}

fn get_starting_nodes<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<&'a str> {
    let mut nodes = Vec::new();

    for line in lines {
        if line[0..3].ends_with('A') {
            nodes.push(&line[0..3]);
        }
    }

    nodes
}

fn get_prime_factorization(number: u64) -> Vec<(u64, u32)> {
    let mut factors = Vec::new();
    let mut current_number = number;

    while current_number > 1 {
        let mut has_found_factor = false;

        for index in 2..current_number {
            if current_number % index == 0 {
                has_found_factor = true;

                factors.push(index);

                current_number /= index;

                break;
            }
        }

        if !has_found_factor {
            factors.push(current_number);
            break;
        }
    }

    factors.sort();

    let mut factors_summarized = Vec::new();

    for factor in factors {
        if let Some((_, count)) = factors_summarized
            .iter_mut()
            .find(|(inner_factor, _)| *inner_factor == factor)
        {
            *count += 1;
        } else {
            factors_summarized.push((factor, 1));
        }
    }

    factors_summarized
}

struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

impl Node<'_> {
    fn new<'a>(left: &'a str, right: &'a str) -> Node<'a> {
        Node { left, right }
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
