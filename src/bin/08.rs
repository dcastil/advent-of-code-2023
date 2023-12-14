use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let navigation_instructions = lines.next().unwrap();
    let nodes = get_nodes_hash_map(lines.skip(1));

    let mut current_node_id = "AAA";
    let mut step_count = 0;

    while current_node_id != "ZZZ" {
        for char in navigation_instructions.chars() {
            step_count += 1;

            let current_node = nodes.get(current_node_id).unwrap();

            current_node_id = match char {
                'L' => current_node.left,
                'R' => current_node.right,
                _ => panic!("Invalid navigation instruction"),
            };

            if current_node_id == "ZZZ" {
                break;
            }
        }
    }

    Some(step_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn get_nodes_hash_map<'a>(lines: impl Iterator<Item = &'a str>) -> HashMap<&'a str, Node<'a>> {
    let mut map = HashMap::new();

    for line in lines {
        map.insert(&line[0..3], Node::new(&line[7..10], &line[12..15]));
    }

    map
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
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
