use std::cmp::{max, min};
use std::fs::read_to_string;

fn area(moves: &Vec<(i64, i64)>) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut integral = 0;
    let mut length = 0;

    for m in moves {
        integral += x * m.1;
        length += m.1.abs() + m.0.abs();
        x = x + m.0;
        y = x + m.1;
    }
    return integral.abs() + 1 + length / 2;
}

pub(crate) fn day18(path: &str, part2: bool) {
    let mut moves: Vec<(i64, i64)> = Vec::new();
    for lines in read_to_string(path).unwrap().lines() {
        let element = lines.split(' ').collect::<Vec<&str>>();

        let dir;
        let l: i64;
        if part2 {
            let s = element[2];
            dir = match s.chars().nth(s.len() - 2).unwrap() {
                '0' => "R",
                '1' => "D",
                '2' => "L",
                '3' => "U",
                _ => panic!("Unsupported")
            };

            l = i64::from_str_radix(&s[2..s.len()-2], 16).unwrap();
        } else {
            dir = element[0];
            l = element[1].parse::<i64>().unwrap()
        }

        match dir {
            "R" => moves.push((0, l)),
            "D" => moves.push((l, 0)),
            "L" => moves.push((0, -1 * l)),
            "U" => moves.push((-1 * l, 0)),
            _ => panic!("Unsupported {}", element[0])
        }
    }

    println!("{}", area(&moves))
}
