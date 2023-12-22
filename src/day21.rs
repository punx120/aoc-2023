use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn parse_map(path: &str) -> Vec<Vec<char>> {
    let mut temp_rows: Vec<Vec<char>> = Vec::new();
    for line in read_to_string(path).unwrap().lines() {
        let chars = line.chars().collect();
        temp_rows.push(chars);
    }
    temp_rows
}

fn find_start(map: &Vec<Vec<char>>) -> (usize, usize) {
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == 'S' {
                return (r, c);
            }
        }
    }
    panic!("Start not found");
}

fn process_points(map: &Vec<Vec<char>>, points: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut next_points = HashSet::new();
    for point in points.iter() {
        if point.0 > 0 {
            let next = (point.0 - 1, point.1);
            if map[next.0][next.1] != '#' {
                next_points.insert(next);
            }
        }
        if point.1 > 0 {
            let next = (point.0, point.1 - 1);
            if map[next.0][next.1] != '#' {
                next_points.insert(next);
            }
        }

        if point.0 < map.len() - 1 {
            let next = (point.0 + 1, point.1);
            if map[next.0][next.1] != '#' {
                next_points.insert(next);
            }
        }

        if point.1 < map[0].len() - 1 {
            let next = (point.0, point.1 + 1);
            if map[next.0][next.1] != '#' {
                next_points.insert(next);
            }
        }
    }

    return next_points;
}

fn normalize(rows: i64, cols: i64, point: (i64, i64), d: (i64, i64)) -> (i64, i64) {
    let mut x: i64;
    let mut y: i64;

    let p = (point.0 + d.0, point.1 + d.1);

    if p.0 < 0 {
        x = rows - p.0.abs() % rows;
        if x == rows {
            x = 0;
        }
    } else {
        x = p.0 % rows;
    }

    if p.1 < 0 {
        y = cols - p.1.abs() % cols;
        if y == cols {
            y = 0;
        }
    } else {
        y = p.1 % rows;
    }

    (x, y)
}

fn get(map: &Vec<Vec<char>>, p: (i64, i64)) -> char {
    let rows = map.len() as i64;
    let cols = map[0].len() as i64;
    let mut x: i64;
    let mut y: i64;

    if p.0 < 0 {
        x = rows - p.0.abs() % rows;
        if x == rows {
            x = 0;
        }
    } else {
        x = p.0 % rows;
    }

    if p.1 < 0 {
        y = cols - p.1.abs() % cols;
        if y == cols {
            y = 0;
        }
    } else {
        y = p.1 % rows;
    }

    map[x as usize][y as usize]
}

fn process_points2(map: &Vec<Vec<char>>, points: &HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut next_points = HashSet::new();
    for point in points.iter() {
        let next = (point.0 - 1, point.1);
        if get(map, next) != '#' {
            next_points.insert(next);
        }
        let next = (point.0, point.1 - 1);
        if get(map, next) != '#' {
            next_points.insert(next);
        }

        let next = (point.0 + 1, point.1);
        if get(map, next) != '#' {
            next_points.insert(next);
        }

        let next = (point.0, point.1 + 1);
        if get(map, next) != '#' {
            next_points.insert(next);
        }
    }

    return next_points;
}

fn process_points3(map: &Vec<Vec<char>>, points: &HashSet<(i64, i64)>, counters: &mut HashMap<(i64, i64), u64>) -> HashSet<(i64, i64)> {
    let rows = map.len() as i64;
    let cols = map[0].len() as i64;

    let mut next_points = HashSet::new();
    for point in points.iter() {
        for d in vec![(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let next = normalize(rows, cols, (point.0, point.1), d);
            if map[next.0 as usize][next.1 as usize] != '#' {
                next_points.insert(next);
            }
        }
    }


    next_points.iter().for_each(|p| {
        let x = counters.entry(*p).or_insert(0);
        *x = *x + 1;
    });

    return next_points;
}

pub(crate) fn day21(path: &str, max_steps: u32) {
    let map = parse_map(path);

    let start: (usize, usize) = find_start(&map);

    println!("start={:?}", start);

    let mut to_visit = process_points(&map, &HashSet::from([start]));

    for i in 0..max_steps {
        println!("{} {}", i + 1, to_visit.len());
        to_visit = process_points(&map, &to_visit);
    }
}

fn test_diff(ans: &Vec<usize>, delta: usize) -> i32 {
    let mut diff = Vec::new();
    for i in delta..ans.len() {
        diff.push(ans[i] - ans[i - delta]);
    }

    let mut diff2: Vec<i32> = Vec::new();
    for i in delta..diff.len() {
        diff2.push(diff[i] as i32 - diff[i - delta] as i32);
    }

    let x = diff2[diff2.len() - 1];
    if diff2[diff2.len() - 20..diff2.len()].iter().all(|e| *e == x) {
        return x;
    } else {
        return 0;
    }
}

pub(crate) fn day21_part2(path: &str, max_steps: usize) {
    let map = parse_map(path);

    let tmp: (usize, usize) = find_start(&map);
    let start = (tmp.0 as i64, tmp.1 as i64);

    let mut to_visit = process_points2(&map, &HashSet::from([start]));
    let mut ans = Vec::new();
    let end = std::cmp::min(map.len() * 3, max_steps);
    for i in 0..end {
        // println!("{i} {}", to_visit.len());
        ans.push(to_visit.len());
        to_visit = process_points2(&map, &to_visit);
    }

    let tmp_step = test_diff(&ans, 131);

    for i in 1..=map.len() {
        let step = test_diff(&ans, i);
        if step > 0 {
            println!("Step is {step} width={i}");

            for j in end..max_steps {
                let v = 2 * ans[j - i] - ans[j - 2 * i] + step as usize;
                if v < ans[ans.len() - 1] {
                    println!("")
                }
                ans.push(v);
            }

            println!("ans is {}", ans[max_steps - 1]);

            break;
        }
    }
}