use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs::read_to_string;

use crate::day17::Dir::{East, North, South, West};


/* Inspired by https://github.com/TimHuisman1703/AdventOfCode/blob/master/2023/Day%2017/aoc17_1.py */

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum Dir {
    North,
    South,
    East,
    West
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Data {
    cost: u32,
    r: i32,
    c: i32,
    dir: Dir,
    mult: u8
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

pub(crate) fn day17(path: &str, max_same_direction: u8, min_same_direction: u8) {
    let mut map: Vec<Vec<u32>> = Vec::new();

    for lines in read_to_string(path).unwrap().lines() {
        map.push(lines.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let mut to_visit: BinaryHeap<Data> = BinaryHeap::new();
    let mut visited : HashSet<(i32, i32, Dir, u8)> = HashSet::new();

    to_visit.push(Data{r: 0, c: 1, cost: map[0][1], dir: East, mult: 1});
    to_visit.push(Data{r: 1, c: 0, cost: map[1][0], dir: South, mult: 1});

    while !to_visit.is_empty() {
        let x = to_visit.pop().unwrap();

        let key = (x.r, x.c, x.dir, x.mult);
        if !visited.insert(key) {
            continue;
        }

        if x.r as usize == map.len() - 1 && x.c as usize == map[0].len() - 1 {
            println!("Done {}", x.cost);
            break;
        }

        for d in vec![North, South, West, East] {
            if x.dir != d && x.mult < min_same_direction {
                continue;
            }
            if x.mult == max_same_direction && x.dir == d {
                continue;
            }

            if match d {
                North => {x.dir == South}
                South => {x.dir == North}
                East => {x.dir == West}
                West => {x.dir == East}
            } {
                continue;
            }


            let (nr, nc) = match d {
                North => { (x.r - 1, x.c) }
                South => { (x.r + 1, x.c) }
                East => { (x.r, x.c + 1) }
                West => { (x.r, x.c - 1) }
            };

            if nr < 0 || nc < 0 || nr as usize >= map.len() || nc as usize >= map[0].len() {
                continue;
            }

            let new_mult = 1 + if d == x.dir { x.mult } else { 0 };
            let cost = x.cost + map[nr as usize][nc as usize];
            to_visit.push(Data { cost, r: nr, c: nc, dir: d, mult: new_mult });
        }
    }
}