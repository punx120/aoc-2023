use std::collections::HashMap;
use std::fs::read_to_string;
use std::thread::panicking;

struct Op {
    attribute: char,
    value: u32,
    op: char,
    target: String,
}

struct Workflow {
    operations: Vec<Op>,
}

fn process(wf: &Workflow, part: &HashMap<char, u32>, workflows: &HashMap<&str, Workflow>) -> u32 {
    for op in wf.operations.iter() {
        let success = match op.op {
            '>' => part.get(&op.attribute).unwrap() > &op.value,
            '<' => part.get(&op.attribute).unwrap() < &op.value,
            '=' => true,
            _ => panic!("Error"),
        };

        if success {
            return match op.target.as_str() {
                "R" => return 0,
                "A" => return part.values().sum(),
                _ => process(workflows.get(op.target.as_str()).unwrap(), part, workflows),
            };
        }
    }
    return 0;
}

pub(crate) fn day19(path: &str) {
    let mut workflows: HashMap<&str, Workflow> = HashMap::new();

    let mut sum = 0u32;
    let mut wf_done = false;
    for line in read_to_string(path).unwrap().lines() {
        if line.is_empty() {
            wf_done = true;
        } else if !wf_done {
            let index = line.find("{").unwrap();
            let name = &line[0..index];
            let parts: Vec<&str> = line[index+1..line.len() - 1].split(',').collect::<Vec<&str>>();

            let operations: Vec<Op> = parts.iter().map(|o| -> Op {
                if o.contains(":") {
                    let (left, right) = o.split_once(":").unwrap();
                    let attr = left.chars().nth(0).unwrap();
                    let op = left.chars().nth(1).unwrap();
                    let value = left[2..left.len()].parse::<u32>().unwrap();
                    return Op { attribute: attr, value: value, op: op, target: right.to_string() };
                } else {
                    return Op { attribute: '\0', value: 0, op: '=', target: o.to_string() };
                }
            }).collect();

            workflows.insert(name, Workflow { operations });
        } else {
            let mut part: HashMap<char, u32> = HashMap::new();
            line[1..line.len() - 1].split(',').for_each(|e| {
                let a = e.chars().nth(0).unwrap();
                let v = e[2..e.len()].parse().unwrap();
                part.insert(a, v);
            });

            sum += process(workflows.get("in").unwrap(), &part, &workflows);
        }
    }

    println!("{sum}");
}