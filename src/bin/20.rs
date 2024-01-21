use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u64> {
    let mut program = Program::from_input(input);
    let mut pulse_count = PulseCount::new();

    let mut add_to_pulse_count = |pulse_packet: &PulsePacket| pulse_count.add(pulse_packet.pulse);

    for _ in 0..1000 {
        program.run(&mut add_to_pulse_count);
    }

    Some(pulse_count.high * pulse_count.low)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut program = Program::from_input(input);

    // Due to the puzzle input shape we know the rx module has a single conjunction input
    let receiver_module = program.module_inputs.get("rx").unwrap()[0];

    let mut runtimes = HashMap::new();

    for index in 0..2_u64.pow(12) {
        program.run(&mut |pulse_packet| {
            if pulse_packet.to == receiver_module && pulse_packet.pulse == Pulse::High {
                runtimes
                    .entry(pulse_packet.from.to_owned())
                    .or_insert(index + 1);
            }
        })
    }

    Some(runtimes.values().product::<u64>())
}

struct Program<'a> {
    modules: HashMap<&'a str, Module<'a>>,
    module_inputs: HashMap<&'a str, Vec<&'a str>>,
}

impl Program<'_> {
    fn from_input(input: &str) -> Program {
        let mut module_outputs = Vec::new();
        let mut modules = HashMap::new();
        let mut module_inputs: HashMap<&str, Vec<&str>> = HashMap::new();

        for line in input.lines() {
            let module = Module::from_line(line);

            module_outputs.push((module.name, module.output_modules.clone()));
            modules.insert(module.name, module);
        }

        for (module_name, outputs) in module_outputs.iter() {
            for output_module in outputs {
                if let Some(module) = modules.get_mut(output_module) {
                    module.add_input(module_name);
                }

                module_inputs
                    .entry(output_module)
                    .or_default()
                    .push(module_name);
            }
        }

        Program {
            modules,
            module_inputs,
        }
    }

    fn run(&mut self, on_pulse_packet: &mut impl FnMut(&PulsePacket)) {
        let mut pulse_packets = VecDeque::from([PulsePacket::start()]);

        while let Some(pulse_packet) = pulse_packets.pop_front() {
            on_pulse_packet(&pulse_packet);

            if let Some(module) = self.modules.get_mut(pulse_packet.to) {
                if let Some(pulse) = module.process(pulse_packet) {
                    pulse_packets.extend(module.output_modules.iter().map(|output_module| {
                        PulsePacket {
                            from: module.name,
                            to: output_module,
                            pulse,
                        }
                    }));
                }
            }
        }
    }
}

struct Module<'a> {
    name: &'a str,
    logic: Logic<'a>,
    output_modules: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn from_line(line: &str) -> Module {
        let mut line_iterator = line.split(" -> ");
        let name_part = line_iterator.next().unwrap();
        let output_modules = line_iterator.next().unwrap().split(", ").collect();

        match name_part.as_bytes()[0] {
            b'%' => Module {
                name: &name_part[1..],
                logic: Logic::new_flip_flop(),
                output_modules,
            },
            b'&' => Module {
                name: &name_part[1..],
                logic: Logic::new_conjunction(),
                output_modules,
            },
            b'b' => Module {
                name: name_part,
                logic: Logic::new_broadcast(),
                output_modules,
            },
            _ => panic!("Invalid module {}", name_part),
        }
    }

    fn add_input(&mut self, input: &'a str) {
        self.logic.add_input(input);
    }

    fn process(&mut self, pulse_packet: PulsePacket) -> Option<Pulse> {
        self.logic.process(pulse_packet)
    }
}

enum Logic<'a> {
    FlipFlop { is_on: bool },
    Conjunction { inputs: Vec<(&'a str, Pulse)> },
    Broadcast,
}

impl<'a> Logic<'a> {
    fn new_flip_flop() -> Logic<'a> {
        Logic::FlipFlop { is_on: false }
    }

    fn new_conjunction() -> Logic<'a> {
        Logic::Conjunction { inputs: Vec::new() }
    }

    fn new_broadcast() -> Logic<'a> {
        Logic::Broadcast
    }

    fn add_input(&mut self, input: &'a str) {
        if let Logic::Conjunction { inputs } = self {
            inputs.push((input, Pulse::Low));
        }
    }

    fn process(&mut self, pulse_packet: PulsePacket) -> Option<Pulse> {
        match self {
            Logic::FlipFlop { is_on } => match pulse_packet.pulse {
                Pulse::High => None,
                Pulse::Low => {
                    *is_on = !*is_on;

                    if *is_on {
                        Some(Pulse::High)
                    } else {
                        Some(Pulse::Low)
                    }
                }
            },
            Logic::Conjunction { inputs } => {
                inputs
                    .iter_mut()
                    .find(|(name, _)| *name == pulse_packet.from)
                    .unwrap()
                    .1 = pulse_packet.pulse;

                if pulse_packet.pulse == Pulse::High
                    && inputs.iter().all(|(_, pulse)| *pulse == Pulse::High)
                {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            Logic::Broadcast => Some(pulse_packet.pulse),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Pulse {
    High,
    Low,
}

struct PulseCount {
    high: u64,
    low: u64,
}

impl PulseCount {
    fn new() -> PulseCount {
        PulseCount { high: 0, low: 0 }
    }

    fn add(&mut self, pulse: Pulse) {
        match pulse {
            Pulse::High => self.high += 1,
            Pulse::Low => self.low += 1,
        }
    }
}

struct PulsePacket<'a> {
    from: &'a str,
    to: &'a str,
    pulse: Pulse,
}

impl<'a> PulsePacket<'a> {
    fn start() -> PulsePacket<'a> {
        PulsePacket {
            from: "button",
            to: "broadcaster",
            pulse: Pulse::Low,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11687500));
    }
}
