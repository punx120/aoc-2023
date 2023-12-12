use std::fs::read_to_string;

pub(crate) fn day09(path: &str) {
    let mut last_sum = 0_i32;
    let mut beg_sum = 0_i32;
    for line in read_to_string(path).unwrap().lines() {
        let (beg, end) = process_line(line);
        beg_sum += beg;
        last_sum += end;
    }

    println!("beg={beg_sum} last={last_sum}")
}

fn process_line(line: &str) -> (i32, i32) {
    let mut end : Vec<i32> = Vec::new();
    let mut beg : Vec<i32> = Vec::new();

    let s: Vec<i32> = line.split(' ').map(|x| x.trim().parse::<i32>().unwrap()).collect();
    beg.push(s[0]);
    end.push(s[s.len()-1]);
    let mut current = s.to_vec();
    loop {
        let mut next: Vec<i32> = Vec::new();

        for i in 0..(current.len() - 1) {
            let diff = current[i + 1] - current[i];
            next.push(diff);
        }

        current.clear();
        next.iter().for_each(|x| current.push(*x));

        if current.iter().all(|x| *x == 0) {
            break;
        }

        beg.push(current[0]);
        end.push(current[current.len() - 1]);
    }

    let mut last = 0_i32;
    let mut first = 0_i32;
    for i in (0..end.len()).rev() {
        last = last + end[i];
        first = beg[i] - first;
    }

    return (first, last);
}