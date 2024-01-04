use anyhow::{bail, Context};
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let modules_data = parse_modules(&input)?;
    let (mut modules, outputs): (HashMap<_, _>, HashMap<_, _>) = modules_data
        .into_iter()
        .map(|(name, module, outputs)| ((name, module), (name, outputs)))
        .unzip();
    let is_real_input = input.contains("-> rx");

    // Wire conjunction modules with their inputs.
    for (name, outs) in outputs.iter() {
        for out_name in outs.iter() {
            if let Some(Module::Conj { mem }) = modules.get_mut(*out_name) {
                mem.insert(name, LOW);
            }
        }
    }

    let mut low_count = 0;
    let mut high_count = 0;
    let mut conj_first_low_emits = HashMap::new();

    for button_push in 1.. {
        let mut pulses = VecDeque::from_iter([("button", "broadcaster", LOW)]);

        while let Some(pulse) = pulses.pop_front() {
            let (src, dst, value) = pulse;
            high_count += value as u64;
            low_count += !value as u64;

            let mut emit_pulse = |value| {
                let new_pulse_src = dst;
                for new_pulse_dst in &outputs[new_pulse_src] {
                    pulses.push_back((new_pulse_src, new_pulse_dst, value))
                }
            };

            let Some(module) = modules.get_mut(dst) else {
                continue;
            };
            match module {
                Module::Broadcaster {} => {
                    emit_pulse(value);
                }
                Module::FlipFlop { state } => {
                    if value == LOW {
                        *state = !*state;
                        emit_pulse(*state);
                    }
                }
                Module::Conj { mem } => {
                    mem.insert(src, value);
                    let emit_val = !mem.values().all(|&v| v == HIGH);
                    emit_pulse(emit_val);
                    if emit_val == LOW && !conj_first_low_emits.contains_key(dst) {
                        conj_first_low_emits.insert(dst, button_push);
                    }
                }
            }
        }

        if button_push == 1000 {
            let part_1_ans = low_count * high_count;
            if is_real_input {
                print!("{part_1_ans} ");
            } else {
                println!("{part_1_ans}");
                break;
            }
        }

        // Note: multi-input conjunctions work like NANDs and single-input conjunctions, aka
        // inverters, work like NOTs.
        //
        // This is *very* ad-hoc for our input module configuration. The "rx" module is fed by a
        // single conjunction module that is in turn fed by a bunch of inverters that are fed by
        // other conjunctions. So, the conjunction that feeds "rx" will emit a low pulse only when
        // all these other conjunctions, in turn, emit a low pulse.
        //
        // We keep track of when these second-level conjunctions emit their first low pulse, and
        // assume that this is their cycle time (because they are in turn fed by a bunch of
        // flip-flops that repeat their cycles). So, we need to know when all these cycles coincide.
        // Luckily, all these cycles are prime-numbers, so multiplying them together we get the
        // super-cycle that makes them all emit low pulses at the same time, and in turn get the
        // first low pulse into "rx".
        //
        // This solution definitely does *not* generalize to other module configurations.
        if is_real_input {
            let first_low_pulse_to_rx: Option<u64> = modules
                .iter()
                .filter(|&(name, module)| {
                    matches!(module, Module::Conj { mem } if mem.len() > 1)
                        && outputs[name] != ["rx"]
                })
                .map(|(name, _module)| conj_first_low_emits.get(name))
                .product();
            // first_low_pulse_to_rx is Some only when all second-level conjugations have emitted
            // their first low pulse.
            if let Some(part_2_ans) = first_low_pulse_to_rx {
                println!("{part_2_ans:?}");
                break;
            }
        }
    }

    Ok(())
}

const LOW: bool = false;
const HIGH: bool = true;

enum Module<'a> {
    Broadcaster {},
    FlipFlop { state: bool },
    Conj { mem: HashMap<&'a str, bool> },
}

fn parse_modules(input: &str) -> aoc::Result<Vec<(&str, Module, Vec<&str>)>> {
    input.lines().map(|line| parse_module(line)).try_collect()
}

fn parse_module(line: &str) -> aoc::Result<(&str, Module, Vec<&str>)> {
    let (name, outputs) = line.split_once(" -> ").context("invalid line")?;
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
        bail!("invalid module name {name}")
    }
}
