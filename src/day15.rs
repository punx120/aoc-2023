use std::collections::HashMap;
use std::fmt::Formatter;
use std::fs::read_to_string;

fn hash(s: &str) -> u64 {
    let mut h: u64 = 0;
    for c in s.as_bytes() {
        h = ((h + *c as u64) * 17) % 256;
    }
    h
}

pub(crate) fn day15(path: &str) {
    let mut ans = 0 as u64;
    for line in read_to_string(path).unwrap().lines() {
        ans += line.split(",").map(|x| hash(x)).sum::<u64>();
    }
    println!("{ans}");
}

struct Lens {
    label: String,
    fl: u8,
}

impl std::fmt::Debug for Lens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.label, self.fl)
    }
}

pub(crate) fn day15_part2(path: &str) {
    let mut ans = 0 as u64;
    let mut map: HashMap<u8, Vec<Lens>> = HashMap::new();
    for line in read_to_string(path).unwrap().lines() {
        for op in line.split(",") {
            if op.contains('=') {
                let (label, fl) = op.split_once('=').unwrap();
                let box_nb = hash(label) as u8;
                let vec = map.entry(box_nb).or_insert_with(|| Vec::new());

                let mut done = false;
                for l in vec.iter_mut() {
                    if l.label == label {
                        l.fl = fl.parse::<u8>().unwrap();
                        done = true;
                    }
                }

                if !done {
                    vec.push(Lens {
                        label: label.to_string(),
                        fl: fl.parse::<u8>().unwrap(),
                    });
                }
            } else {
                let (label, _) = op.split_once('-').unwrap();
                let box_nb = hash(label) as u8;
                let entry = map.get_mut(&box_nb);
                if entry.is_some() {
                    let v = entry.unwrap();
                    v.retain(|x| x.label != label);
                }
            }
        }
    }

    for (box_nb, v) in &map {
        let b: u64 = *box_nb as u64 + 1;
        for i in 0..v.len() {
            let fp = b * (i as u64 + 1) * v[i].fl as u64;
            ans += fp;
        }
    }

    println!("{ans}");
}