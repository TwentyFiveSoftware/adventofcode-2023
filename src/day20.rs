use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/20.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
}

fn part1(lines: &Vec<String>) -> usize {
    let mut modules = lines
        .iter()
        .map(|line| Module::from_str(line).unwrap())
        .fold(HashMap::new(), |mut map, module| {
            map.insert(module.name.to_string(), module);
            map
        });

    let inputs_per_module = modules.values()
        .fold(HashMap::new(), |mut map, module| {
            let inputs = modules.values()
                .filter(|other_module| other_module.outputs.contains(&module.name))
                .map(|other_module| other_module.name.to_string())
                .collect::<Vec<String>>();

            map.insert(module.name.to_string(), inputs);
            map
        });

    for module in modules.values_mut() {
        if let ModuleType::Conjunction(inputs) = &mut module.module_type {
            for input_module_name in inputs_per_module.get(&module.name).unwrap() {
                inputs.insert(input_module_name.to_string(), PulseType::LOW);
            }
        }
    }

    //

    let mut pulse_counts = HashMap::new();
    pulse_counts.insert(PulseType::LOW, 0);
    pulse_counts.insert(PulseType::HIGH, 0);

    for _ in 0..1000 {
        let mut queue = VecDeque::new();

        queue.push_back(Pulse { from: "".to_string(), to: "broadcaster".to_string(), pulse_type: PulseType::LOW });
        pulse_counts.entry(PulseType::LOW).and_modify(|count| *count += 1);

        while let Some(pulse) = queue.pop_front() {
            if !modules.contains_key(&pulse.to) {
                continue;
            }

            let module = modules.get_mut(&pulse.to).unwrap();
            for new_pulse in module.simulate_pulse(&pulse) {
                pulse_counts.entry(new_pulse.pulse_type).and_modify(|count| *count += 1);
                queue.push_back(new_pulse);
            }
        }
    }

    pulse_counts.values().fold(1, |acc, n| acc * n)
}

struct Module {
    name: String,
    outputs: Vec<String>,
    module_type: ModuleType,
}

enum ModuleType {
    Broadcast,
    FlipFlop(FlipFlowStatus),
    Conjunction(HashMap<String, PulseType>),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum PulseType {
    LOW,
    HIGH,
}

impl FromStr for Module {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(" -> ").collect::<Vec<&str>>();
        let left = split.get(0).unwrap();
        let outputs = split.get(1).unwrap().split(", ")
            .map(|output| output.to_string()).collect::<Vec<String>>();

        let module_type = match () {
            _ if left.starts_with("%") => ModuleType::FlipFlop(FlipFlowStatus::OFF),
            _ if left.starts_with("&") => ModuleType::Conjunction(HashMap::new()),
            _ => ModuleType::Broadcast
        };

        Ok(Module {
            name: left.trim_matches(|c| c == '%' || c == '&').to_string(),
            outputs,
            module_type,
        })
    }
}

impl Module {
    fn simulate_pulse(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        let output_pulse_type = match &mut self.module_type {
            ModuleType::Broadcast => {
                Some(pulse.pulse_type)
            }
            ModuleType::FlipFlop(status) => {
                match pulse.pulse_type {
                    PulseType::LOW => {
                        *status = status.get_inverse();

                        match status {
                            FlipFlowStatus::ON => Some(PulseType::HIGH),
                            FlipFlowStatus::OFF => Some(PulseType::LOW),
                        }
                    }
                    PulseType::HIGH => {
                        None
                    }
                }
            }
            ModuleType::Conjunction(memory) => {
                memory.insert(pulse.from.to_string(), pulse.pulse_type);

                if memory.values().all(|pulse_type| *pulse_type == PulseType::HIGH) {
                    Some(PulseType::LOW)
                } else {
                    Some(PulseType::HIGH)
                }
            }
        };

        return match output_pulse_type {
            None => vec![],
            Some(pulse_type) =>
                self.outputs
                    .iter()
                    .map(|output| Pulse {
                        from: self.name.to_string(),
                        to: output.to_string(),
                        pulse_type,
                    })
                    .collect()
        };
    }
}

#[derive(Copy, Clone)]
enum FlipFlowStatus {
    ON,
    OFF,
}

impl FlipFlowStatus {
    fn get_inverse(&self) -> FlipFlowStatus {
        match self {
            FlipFlowStatus::ON => FlipFlowStatus::OFF,
            FlipFlowStatus::OFF => FlipFlowStatus::ON,
        }
    }
}


struct Pulse {
    from: String,
    to: String,
    pulse_type: PulseType,
}
