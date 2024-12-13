use std::ops::Not;
use std::collections::{HashMap, hash_map::Entry, VecDeque};
use std::str::FromStr;

use num::Integer;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    On, Off,
}

impl Not for State {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            State::On  => State::Off,
            State::Off => State::On,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High, Low,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module {
    FlipFlop(State),
    Conjunction{
        inputs: Vec<usize>,
        memory: Vec<Pulse>,
    },
    Broadcast,
}

#[derive(Debug, Clone)]
struct Machines {
    modules: Vec<(Module, Vec<usize>)>,
    name_lookup: HashMap<String, usize>,
}

impl Machines {
    fn spam_button(&mut self, times: u32) -> u32 {
        let mut high_pulses = 0;
        let mut low_pulses = 0;

        for _ in 0..times {
            let (h, l) = self.button_press(None);
            high_pulses += h;
            low_pulses += l;
        }

        high_pulses * low_pulses
    }

    fn button_press(&mut self, pulsed_high: Option<usize>) -> (u32, u32) {
        let mut pulse_queue = VecDeque::new();
        pulse_queue.push_back((self.name_lookup["broadcaster"], self.name_lookup["broadcaster"], Pulse::Low));

        let mut low_pulses = 0;
        let mut high_pulses = 0;

        while let Some((from, module_index, pulse)) = pulse_queue.pop_front() {
            let (module, sends_to) = &mut self.modules[module_index];
            if let Some(i) = pulsed_high {
                if i == from {
                    match pulse {
                        Pulse::High => high_pulses += 1,
                        Pulse::Low => low_pulses += 1,
                    }
                }
            } else {
                match pulse {
                    Pulse::High => high_pulses += 1,
                    Pulse::Low => low_pulses += 1,
                }
            }

            let new_pulse;
            match module {
                Module::FlipFlop(state) => {
                    let state = *state;

                    if pulse == Pulse::High {
                        continue;
                    } else {
                        *module = Module::FlipFlop(!state);
                        new_pulse = match state {
                            State::On => Pulse::Low,
                            State::Off => Pulse::High,
                        };
                    }
                },
                Module::Conjunction { inputs, memory } => {
                    let index = inputs.iter().position(|module| *module == from).unwrap();
                    memory[index] = pulse;

                    new_pulse =
                        if memory.iter().all(|p| *p == Pulse::High) {
                            Pulse::Low
                        } else {
                            Pulse::High
                        };
                },
                Module::Broadcast => new_pulse = pulse,
            }

            for s in sends_to {
                pulse_queue.push_back((module_index, *s, new_pulse));
            }
        }

        (high_pulses, low_pulses)
    }
}

impl FromStr for Machines {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut modules = Vec::new();
        let mut name_lookup = HashMap::new();

        for l in s.lines() {
            let [module, sends_to] = l.split(" -> ").collect::<Vec<_>>()
                .try_into().map_err(|_| "Invalid module format")?;

            let (index, module) =
                if module == "broadcaster" {
                    name_lookup.insert(module.to_owned(), modules.len());
                    modules.push((Module::Broadcast, Vec::new()));

                    (modules.len() - 1, Module::Broadcast)
                } else if let Some(name) = module.strip_prefix('%') {
                    let index = match name_lookup.entry(name.to_owned()) {
                        Entry::Occupied(e) => *e.get(),
                        Entry::Vacant(e) => {
                            e.insert(modules.len());
                            modules.push((Module::Broadcast, Vec::new()));
                            modules.len() - 1
                        },
                    };

                    (index, Module::FlipFlop(State::Off))
                } else if let Some(name) = module.strip_prefix('&') {
                    let index = match name_lookup.entry(name.to_owned()) {
                        Entry::Occupied(e) => *e.get(),
                        Entry::Vacant(e) => {
                            e.insert(modules.len());
                            modules.push((Module::Broadcast, Vec::new()));
                            modules.len() - 1
                        },
                    };

                    (index, Module::Conjunction{ inputs: Vec::new(), memory: Vec::new() })
                } else {
                    return Err("Invalid module");
                };

            // Sends to
            let sends_to = sends_to.split(", ")
                .map(|st| {
                    match name_lookup.entry(st.to_owned()) {
                        Entry::Occupied(e) => *e.get(),
                        Entry::Vacant(e) => {
                            e.insert(modules.len());
                            modules.push((Module::Broadcast, Vec::new()));
                            modules.len() - 1
                        },
                    }
                })
                .collect();

            modules[index] = (module, sends_to);
        }

        // Set conjunction inputs
        for i in 0..modules.len() {
            if !matches!(modules[i].0, Module::Conjunction { .. }) {
                continue;
            }

            let mut conjunction_inputs = Vec::new();
            for (j, module) in modules.iter().enumerate() {
                if module.1.contains(&i) {
                    conjunction_inputs.push(j);
                }
            }

            if let Module::Conjunction { inputs, memory } = &mut modules[i].0 {
                *inputs = conjunction_inputs;
                *memory = vec![Pulse::Low; inputs.len()];
            } else {
                unreachable!();
            }
        }

        Ok(Self { modules, name_lookup })
    }
}


fn main() {
    let machines = Machines::from_str(include_str!("../input.txt")).unwrap();

    let mut part1_machines = machines.clone();
    println!("[Part 1] Pulses: {}", part1_machines.spam_button(1_000));


    let (output_conjunction, _) = machines.modules.iter()
        .find(|(_, st)| st.contains(&machines.name_lookup["rx"]))
        .unwrap();
    let receives_from = match output_conjunction {
        Module::FlipFlop(_) => panic!("Not a conjunction"),
        Module::Conjunction { inputs, .. } => inputs,
        Module::Broadcast => panic!("Not a conjunction"),
    };

    let mut high_pulse_rates = Vec::new();
    for r in receives_from {
        let mut button_presses: u64 = 0;
        let mut machine = machines.clone();
        loop {
            let (h, _) = machine.button_press(Some(*r));
            button_presses += 1;

            if h > 0 {
                high_pulse_rates.push(button_presses);
                break;
            }
        }
    }

    let required_button_presses = high_pulse_rates.into_iter()
        .fold(1, |acc, r| acc.lcm(&r));
    println!("[Part 2] Required button presses: {required_button_presses}");
}
