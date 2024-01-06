use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let mut input_iterator = input.split("\n\n");
    let workflow_map = WorkflowMap::from_input(input_iterator.next().unwrap());

    let sum = input_iterator
        .next()
        .unwrap()
        .lines()
        .map(Object::from_line)
        .filter_map(|object| {
            if workflow_map.accepts(&object) {
                Some(object.x + object.m + object.a + object.s)
            } else {
                None
            }
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

struct WorkflowMap<'a> {
    workflows: HashMap<&'a str, Workflow<'a>>,
}

impl WorkflowMap<'_> {
    fn from_input(input: &str) -> WorkflowMap {
        WorkflowMap {
            workflows: input
                .lines()
                .map(Workflow::from_line)
                .map(|workflow| (workflow.name, workflow))
                .collect(),
        }
    }

    fn accepts(&self, object: &Object) -> bool {
        let mut command = &Command::Forward("in");

        while let Command::Forward(workflow_name) = command {
            command = self.next_command(workflow_name, &object);
        }

        match command {
            Command::Accept => true,
            Command::Reject => false,
            _ => panic!("Invalid command"),
        }
    }

    fn next_command(&self, workflow_name: &str, object: &Object) -> &Command {
        self.workflows
            .get(workflow_name)
            .unwrap()
            .next_command(object)
    }
}

struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
    final_command: Command<'a>,
}

impl Workflow<'_> {
    fn from_line(line: &str) -> Workflow {
        let mut line_iterator = line.split('{');

        let name = line_iterator.next().unwrap();
        let rules_string = line_iterator.next().unwrap();

        let mut rules_iterator = rules_string[..rules_string.len() - 1].split(',');

        let final_command = Command::from_string(rules_iterator.next_back().unwrap());
        let mut rules: Vec<_> = rules_iterator.map(Rule::from_string).collect();

        // Optimization that removes unnecessary rules
        while rules
            .last()
            .map_or(false, |rule| rule.command == final_command)
        {
            rules.pop();
        }

        Workflow {
            name,
            rules,
            final_command,
        }
    }

    fn next_command(&self, object: &Object) -> &Command {
        self.rules
            .iter()
            .find(|rule| rule.applies_to(object))
            .map_or(&self.final_command, |rule| &rule.command)
    }
}

struct Rule<'a> {
    condition: Condition,
    command: Command<'a>,
}

impl Rule<'_> {
    fn from_string(string: &str) -> Rule {
        let mut string_iterator = string.split(':');

        Rule {
            condition: Condition::from_string(string_iterator.next().unwrap()),
            command: Command::from_string(string_iterator.next().unwrap()),
        }
    }

    fn applies_to(&self, object: &Object) -> bool {
        self.condition.ordering
            == object
                .value(&self.condition.property)
                .cmp(&self.condition.cmp_value)
    }
}

struct Condition {
    property: Property,
    ordering: Ordering,
    cmp_value: u32,
}

impl Condition {
    fn from_string(string: &str) -> Condition {
        let property = match &string[0..1] {
            "x" => Property::X,
            "m" => Property::M,
            "a" => Property::A,
            "s" => Property::S,
            _ => panic!("Invalid property {}", string),
        };

        let ordering = match &string[1..2] {
            "<" => Ordering::Less,
            ">" => Ordering::Greater,
            _ => panic!("Invalid ordering"),
        };

        Condition {
            property,
            ordering,
            cmp_value: string[2..].parse().unwrap(),
        }
    }
}

enum Property {
    X,
    M,
    A,
    S,
}

#[derive(PartialEq)]
enum Command<'a> {
    Accept,
    Reject,
    Forward(&'a str),
}

impl Command<'_> {
    fn from_string(string: &str) -> Command {
        match string {
            "A" => Command::Accept,
            "R" => Command::Reject,
            _ => Command::Forward(string),
        }
    }
}

struct Object {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Object {
    fn from_line(line: &str) -> Object {
        let mut line_iterator = line[1..line.len() - 1].split(',');

        Object {
            x: line_iterator.next().unwrap()[2..].parse().unwrap(),
            m: line_iterator.next().unwrap()[2..].parse().unwrap(),
            a: line_iterator.next().unwrap()[2..].parse().unwrap(),
            s: line_iterator.next().unwrap()[2..].parse().unwrap(),
        }
    }

    fn value(&self, property: &Property) -> u32 {
        match property {
            Property::X => self.x,
            Property::M => self.m,
            Property::A => self.a,
            Property::S => self.s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
