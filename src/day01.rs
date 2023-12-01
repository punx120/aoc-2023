use std::fs::read_to_string;

pub(crate) fn day01_part1(path: &str) {
    let mut result = 0;

    for line in read_to_string(path).unwrap().lines() {
        let mut first = None;
        let mut second = None;
        for c in line.to_string().chars() {
            if c.is_numeric() {
                if first.is_none() {
                    first = c.to_digit(10);
                } else {
                    second = c.to_digit(10);
                }
            }
        }

        let i = first.unwrap() * 10 + second.unwrap_or_else(|| first.unwrap());
        result += i;
    }

    println!("{:?}", result);
}

pub(crate) fn day01_part2(path: &str) {
    let mut result = 0;

    for line in read_to_string(path).unwrap().lines() {
        let mut first = None;
        let mut last = None;

        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if first.is_none() {
                    first = c.to_digit(10);
                } else {
                    last = c.to_digit(10);
                }
            } else {
                let digit = read_digit(line, i);
                if digit.is_some() {
                    if first.is_none() {
                        first = digit;
                    } else {
                        last = digit;
                    }
                }
            }
        }
        let i = first.unwrap() * 10 + last.unwrap_or_else(|| first.unwrap());
        println!("{:?}", i);
        result += i;
    }

    fn read_digit(s: &str, idx: usize) -> Option<u32> {
        let x = s.get(idx..).unwrap();
        return if x.starts_with("one") {
            Some(1)
        } else if x.starts_with("two") {
            Some(2)
        } else if x.starts_with("three") {
            Some(3)
        } else if x.starts_with("four") {
            Some(4)
        } else if x.starts_with("five") {
            Some(5)
        } else if x.starts_with("six") {
            Some(6)
        } else if x.starts_with("seven") {
            Some(7)
        } else if x.starts_with("eight") {
            Some(8)
        } else if x.starts_with("nine") {
            Some(9)
        } else {
            None
        }
    }

    println!("{:?}", result);
}