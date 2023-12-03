#![allow(dead_code)]

use std::cmp::{max, min};
use std::fs::read_to_string;

pub fn is_symbol(line: &str, idx: usize) -> bool {
    let char = line.chars().nth(idx).unwrap_or('.');
    return char != '.' && !char.is_numeric();
}

pub fn is_next_to_symbol(prev: Option<&str>, current: Option<&str>, next: Option<&str>, start: usize, end: usize) -> bool {
    // Current line
    if is_symbol(current.unwrap(), end) || (start > 0 && is_symbol(current.unwrap(), start - 1)) {
        return true;
    }

    if !prev.is_none() && check(prev.unwrap(), start, end) {
        return true;
    }

    if !next.is_none() && check(next.unwrap(), start, end) {
        return true;
    }

    return false;
}

fn process_line(prev: Option<&str>, current: Option<&str>, next: Option<&str>, sum: &mut i32) {
    let mut start = None;
    for (i, c) in current.unwrap().chars().enumerate() {
        if !c.is_numeric() {
            if !start.is_none() {
                check_number(prev, current, next, sum, start, i);
                start = None;
            }
        } else if c.is_numeric() && start.is_none() {
            start = Some(i);
        }
    }

    if !start.is_none() {
        let i = current.unwrap().len();
        check_number(prev, current, next, sum, start, i);
    }
}

fn check_number(prev: Option<&str>, current: Option<&str>, next: Option<&str>, sum: &mut i32, start: Option<usize>, i: usize) {
    let nb = &current.unwrap()[start.unwrap()..i].parse::<i32>().unwrap();
    if is_next_to_symbol(prev, current, next, start.unwrap(), i) {
        *sum += nb;
    }
}

fn check(line: &str, start: usize, end: usize) -> bool {
    let from = max((start as i32) - 1, 0) as usize;
    let to = min(end + 1, line.len() + 1);
    for i in from..to {
        if is_symbol(line, i) {
            return true;
        }
    }
    return false;
}

pub(crate) fn day03(path: &str) {
    let mut prev: Option<&str> = None;
    let mut current: Option<&str> = None;
    let mut next: Option<&str> = None;

    let mut sum = 0;

    let lines = read_to_string(path).unwrap();
    for line in lines.lines() {
        if !current.is_none() {
            prev = current;
        }

        if !next.is_none() {
            current = next;
        }

        next = Some(line);

        if !current.is_none() {
            process_line(prev, current, next, &mut sum);
        }
    }

    prev = current;
    current = next;
    next = None;

    process_line(prev, current, next, &mut sum);

    println!("{:?}", sum);
}

pub(crate) fn day03_part2(path: &str) {
    let mut prev: Option<&str> = None;
    let mut current: Option<&str> = None;
    let mut next: Option<&str> = None;

    let mut sum = 0;

    let lines = read_to_string(path).unwrap();
    for line in lines.lines() {
        if !current.is_none() {
            prev = current;
        }

        if !next.is_none() {
            current = next;
        }

        next = Some(line);

        if !current.is_none() {
            for (i, c) in current.unwrap().chars().enumerate() {
                if c == '*' {
                    sum += process_gear_line(prev, current.unwrap(), next, i);
                }
            }
        }
    }

    prev = current;
    current = next;
    next = None;

    for (i, c) in current.unwrap().chars().enumerate() {
        if c == '*' {
            sum += process_gear_line(prev, current.unwrap(), next, i);
        }
    }

    println!("{:?}", sum);
}

fn process_gear_line(prev: Option<&str>, current: &str, next: Option<&str>, i: usize) -> u32 {
    let mut first: Option<u32> = None;
    let mut second: Option<u32> = None;

    // Current row
    if current.chars().nth(i + 1).unwrap_or(' ').is_numeric() {
        if first.is_none() {
            find_forward(current, i + 1, &mut first);
        } else if second.is_none() {
            find_forward(current, i + 1, &mut second);
        } else {
            return 0;
        }
    }

    if i > 0 && current.chars().nth(i - 1).unwrap().is_numeric() {
        if first.is_none() {
            find_backward(current, i - 1, &mut first);
        } else if second.is_none() {
            find_backward(current, i - 1, &mut second);
        } else {
            return 0;
        }
    }

    // prev row
    if !prev.is_none() {
        if !find_numbers(i, prev.unwrap(), &mut first, &mut second) {
            return 0;
        }
    }

    if !next.is_none() {
        if !find_numbers(i, next.unwrap(), &mut first, &mut second) {
            return 0;
        }
    }

    // println!("{:?} {:?} {:?}", current, first, second);

    if !first.is_none() && !second.is_none() {
        return first.unwrap() * second.unwrap();
    }

    return 0;
}

fn find_numbers(i: usize, line: &str, first: &mut Option<u32>, second: &mut Option<u32>) -> bool {
    if line.chars().nth(i).unwrap().is_numeric() {
        let mut start: usize = i;
        for i in (0..i+1).rev() {
            if line.chars().nth(i).unwrap().is_numeric() {
                start = i;
            } else {
                break
            }
        }

        if first.is_none() {
            find_forward(line, start, first);
        } else if second.is_none() {
            find_forward(line, start, second);
        } else {
            return false;
        }
    } else {
        if i > 0 && line.chars().nth(i - 1).unwrap().is_numeric() {
            if first.is_none() {
                find_backward(line, i - 1, first);
            } else if second.is_none() {
                find_backward(line, i - 1, second);
            } else {
                return false;
            }
        }

        if line.chars().nth(i + 1).unwrap_or(' ').is_numeric() {
            if first.is_none() {
                find_forward(line, i + 1, first);
            } else if second.is_none() {
                find_forward(line, i + 1, second);
            } else {
                return false;
            }
        }
    }

    return true;
}

fn find_backward(line: &str, idx: usize, output: &mut Option<u32>) {
    for i in (0..idx).rev() {
        if !line.chars().nth(i).unwrap().is_numeric() {
            let _ = output.insert(line[i + 1..idx + 1].parse::<u32>().unwrap());
            return;
        }
    }

    let _ = output.insert(line[0..idx + 1].parse::<u32>().unwrap());
}

fn find_forward(line: &str, idx: usize, output: &mut Option<u32>) {
    for i in idx..line.len() {
        if !line.chars().nth(i).unwrap().is_numeric() {
            let _ = output.insert(line[idx..i].parse::<u32>().unwrap());
            return;
        }
    }
    let _ = output.insert(line[idx..line.len()].parse::<u32>().unwrap());
}
