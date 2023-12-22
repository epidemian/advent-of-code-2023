use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let modules_data = parse_modules(&input)?;
    let (mut modules, outputs): (HashMap<_, _>, HashMap<_, _>) = modules_data
        .into_iter()
        .map(|(name, module, outputs)| ((name, module), (name, outputs)))
        .unzip();

    // Connect conjunction modules with their inputs.
    for (name, outs) in outputs.iter() {
        for out_name in outs.iter() {
            if let Some(Module::Conj { mem }) = modules.get_mut(*out_name) {
                mem.insert(name.to_string(), false);
            }
        }
    }
    let mut low_count = 0;
    let mut high_count = 0;

    for _ in 0..1000 {
        let mut pulses = VecDeque::from_iter([("button", "broadcaster", LOW)]);

        while let Some(pulse) = pulses.pop_front() {
            let (src, dst, value) = pulse;
            // println!("{src} -{value}-> {dst}");
            if value {
                high_count += 1;
            } else {
                low_count += 1;
            }

            let Some(module) = modules.get_mut(dst) else {
                continue;
            };
            match module {
                Module::Broadcaster {} => {
                    for pulse_dst in &outputs[dst] {
                        pulses.push_back((dst, pulse_dst, value))
                    }
                }
                Module::FlipFlop { state } => {
                    if value == LOW {
                        *state = !*state;
                        for pulse_dst in &outputs[dst] {
                            pulses.push_back((dst, pulse_dst, *state))
                        }
                    }
                }
                Module::Conj { mem } => {
                    *mem.get_mut(src).unwrap() = value;
                    let emit_value = !mem.values().all(|&v| v == HIGH);
                    for pulse_dst in &outputs[dst] {
                        pulses.push_back((dst, pulse_dst, emit_value))
                    }
                }
            }
        }
    }
    println!("{}", low_count * high_count);

    Ok(())
}

const LOW: bool = false;
const HIGH: bool = true;

#[derive(Debug)]
enum Module {
    Broadcaster {},
    FlipFlop { state: bool },
    Conj { mem: HashMap<String, bool> },
}

fn parse_modules(input: &str) -> aoc::Result<Vec<(&str, Module, Vec<&str>)>> {
    input.lines().map(|line| parse_module(line)).try_collect()
}

fn parse_module(line: &str) -> aoc::Result<(&str, Module, Vec<&str>)> {
    let (name, outputs) = line.split_once(" -> ").ok_or("invalid line")?;
    let outputs = outputs.split(", ").collect();
    if name == "broadcaster" {
        Ok((name, Module::Broadcaster {}, outputs))
    } else if let Some(name) = name.strip_prefix('%') {
        Ok((name, Module::FlipFlop { state: false }, outputs))
    } else if let Some(name) = name.strip_prefix('&') {
        let module = Module::Conj {
            mem: HashMap::new(),
        };
        Ok((name, module, outputs))
    } else {
        Err(format!("invalid module name {name}"))?
    }
}
