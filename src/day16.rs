use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Point {
    r: i32,
    c: i32,
}

impl Point {
    pub fn move_point(self, m: Move) -> Point {
        let new_row = self.r + m.h;
        let new_col = self.c + m.v;
        return Point { r: new_row, c: new_col };
    }

    pub fn is_valid(self, lines: &Vec<Vec<char>>) -> bool {
        self.r >= 0 && self.c >= 0 && (self.r as usize) < lines.len() && (self.c as usize) < lines[0].len()
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Move {
    h: i32,
    v: i32,
}

fn process(new_point: Point, m: Move, lines: &Vec<Vec<char>>) -> Vec<(Point, Move)> {
    return match lines[new_point.r as usize][new_point.c as usize] {
        '.' => vec![(new_point, m)],
        '|' => {
            if m.h == 0 { // Split...
                vec![(new_point, Move { h: 1, v: 0 }), (new_point, Move { h: -1, v: 0 })]
            } else {
                vec![(new_point, m)]
            }
        }
        '-' => {
            if m.v == 0 {
                vec![(new_point, Move { h: 0, v: 1 }), (new_point, Move { h: 0, v: -1 })]
            } else {
                vec![(new_point, m)]
            }
        }
        '\\' => {
            if m.v == 1 {
                vec![(new_point, Move { h: 1, v: 0 })]
            } else if m.v == -1 {
                vec![(new_point, Move { h: -1, v: 0 })]
            } else if m.h == -1 {
                vec![(new_point, Move { h: 0, v: -1 })]
            } else if m.h == 1 {
                vec![(new_point, Move { h: 0, v: 1 })]
            } else {
                panic!("Invalid move {:?}", m)
            }
        }
        '/' => {
            if m.v == 1 {
                vec![(new_point, Move { h: -1, v: 0 })]
            } else if m.v == -1 {
                vec![(new_point, Move { h: 1, v: 0 })]
            } else if m.h == -1 {
                vec![(new_point, Move { h: 0, v: 1 })]
            } else if m.h == 1 {
                vec![(new_point, Move { h: 0, v: -1 })]
            } else {
                panic!("Invalid move {:?}", m)
            }
        }
        _ => panic!("Unknown value")
    };
}

pub(crate) fn day16_part2(path: &str) {
    let lines = read(path);
    let mut max = 0;

    let num_rows = lines.len();
    let num_cols = lines[0].len();

    let mut options: Vec<(Point, Move)> = Vec::new();
    for r in 0..num_rows {
        options.push((Point { r: r as i32, c: -1 }, Move { h: 0, v: 1 }));
        options.push((Point { r: r as i32, c: num_cols as i32 }, Move { h: 0, v: -1 }));
    }

    for c in 0..num_cols {
        options.push((Point { r: -1, c: c as i32 }, Move { h: 1, v: 0 }));
        options.push((Point { r: num_rows as i32, c: c as i32 }, Move { h: -1, v: 0 }));
    }


    for (p,m) in options {
        let e = solve_for(&lines, p, m);
        if e > max {
            max = e;
        }
    }

    println!("{max}")
}

fn solve_for(lines: &Vec<Vec<char>>, p: Point, m: Move) -> usize {
    let mut visited: HashSet<(Point, Move)> = HashSet::new();
    let mut to_visit: VecDeque<(Point, Move)> = VecDeque::new();

    to_visit.push_back((p, m));

    while !to_visit.is_empty() {
        let (p, m) = to_visit.pop_front().unwrap();
        let new_point = p.move_point(m);
        if new_point.is_valid(&lines) {
            if visited.insert((new_point, m)) {
                let next = process(new_point, m, &lines);
                next.iter().for_each(|x| {
                    if x.0.is_valid(&lines) {
                        to_visit.push_back(*x);
                    }
                });
            }
        }
    }

    let points: HashSet<Point> = visited.iter().map(|x| x.0).collect();
    points.len()
}

pub(crate) fn day16(path: &str) {
    let lines = read(path);
    println!("{}", solve_for(&lines, Point {r: 0, c: -1}, Move {h:0, v:1}));
}

fn read(path: &str) -> Vec<Vec<char>> {
    let mut lines: Vec<Vec<char>> = Vec::new();
    for line in read_to_string(path).unwrap().lines() {
        lines.push(line.chars().collect());
    }
    lines
}

fn display_visited(visited: &HashSet<Point>, lines: &Vec<Vec<char>>) {
    print!("  ");
    for c in 0..lines[0].len() {
        print!("{c}");
    }
    print!("\n");

    for r in 0..lines.len() {
        print!("{r} ");
        for c in 0..lines[0].len() {
            let point = Point { r: r as i32, c: c as i32 };
            print!("{}", if visited.contains(&point) { '#' } else { '.' });
        }
        print!("\n");
    }
}