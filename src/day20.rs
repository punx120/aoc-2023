use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

use Pulse::Low;

use crate::day20::Pulse::High;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum Pulse {
    Low,
    High,
}

#[derive(Eq, PartialEq, Debug)]
struct Module {
    targets: Vec<String>,
    state: Option<Pulse>,
    modtype: char,
    on: bool,
    inputs: HashMap<String, Pulse>,
}

/*impl Copy for Module {
}*/

impl Clone for Module {
    fn clone(&self) -> Self {
        Module {
            targets: self.targets.clone(),
            state: self.state.clone(),
            modtype: self.modtype,
            on: self.on,
            inputs: self.inputs.clone(),
        }
    }
}

impl Module {
    fn new(targets: Vec<String>, state: Option<Pulse>, modtype: char) -> Self {
        Module { targets: targets, state: state, modtype: modtype, on: false, inputs: HashMap::new() }
    }

    fn process(&mut self, pulse: Pulse) -> Option<Pulse> {
        if self.modtype == '&' {
            if pulse == Low {
                if self.on {
                    self.on = false;
                    return Some(Low);
                } else {
                    self.on = true;
                    return Some(High);
                }
            } else {
                return None;
            }
        } else if self.modtype == '&' {}
        return None;
    }
}

pub(crate) fn day20(path: &str) {
    let mut config = HashMap::new();

    let mut conjunction_modules = HashSet::new();

    for lines in read_to_string(path).unwrap().lines() {
        let (mut left, right) = lines.split_once(" -> ").unwrap();

        let targets: Vec<String> = right.split(",").map(|x| x.trim().to_string()).collect();
        left = left.trim();
        let modtype = left.chars().nth(0).unwrap();
        if modtype == '&' || modtype == '%' {
            let name = left[1..left.len()].to_string();
            if modtype == '&' {
                conjunction_modules.insert(name.to_string());
            }
            config.insert(name, Module::new(targets, None, '&'));
        } else {
            let name = left[0..left.len()].to_string();
            let state = if name == "broadcaster" { Some(Low) } else { None };
            config.insert(name, Module::new(targets, state, '\0'));
        }
    }

    /*for (n, m) in config.iter() {
        for t in m.targets.iter() {
            if conjunction_modules.contains(t) {
                let o = config.get(t).unwrap();
                o.inputs.insert(n.to_string(), Low);
                // config.get_mut(t).unwrap().inputs.insert(n.to_string(), Low);
            }
        }
    }*/

    let mut low_count = 1u64;
    let mut high_count = 0u64;

    let mut todo: VecDeque<(&str, Pulse)> = VecDeque::new();
    todo.push_back(("broadcaster", Low));
    while !todo.is_empty() {
        let (name, pulse) = todo.pop_front().unwrap();
        let module = config.get_mut(name).unwrap();
        let next_pulse = module.process(pulse);
        if !next_pulse.is_none() {
            if next_pulse.unwrap() == High {
                high_count += 1;
            } else {
                low_count += 1;
            }
            module.targets.iter().for_each(|t| todo.push_back((t, next_pulse.unwrap())));
        }
    }

    println!("{high_count}x{low_count}={}", low_count * high_count);
}