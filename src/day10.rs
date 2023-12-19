use std::collections::HashSet;
use std::fs::read_to_string;
use crate::day10::Dir::{East, North, South, West};

fn find_start(rows: &Vec<Vec<char>>) -> ((usize, usize), char) {
    for r in 0..rows.len() {
        for c in 0..rows[r].len() {
            if rows[r][c] == 'S' {
                let point = (r, c);
                let north = r > 0 && Vec::from(['|', 'F', '7']).contains(&rows[r - 1][c]);
                let south = r < rows.len() - 1 && Vec::from(['|', 'L', 'J']).contains(&rows[r + 1][c]);
                let east = c < rows[0].len() - 1 && Vec::from(['-', 'J', '7']).contains(&rows[r][c + 1]);
                let west = c > 0 && Vec::from(['-', 'L', 'F']).contains(&rows[r][c-1]);
                return (point, get_tile(north, south, east, west));
            }
        }
    }
    panic!("Unable to find start!");
}

fn get_tile(north: bool, south: bool, east: bool, west: bool) -> char {
    if north {
        if south {
            return '|';
        } else if east {
            return 'L';
        } else if west {
            return 'J';
        }
    } else if south {
        if east {
            return 'F';
        } else if west {
            return '7';
        }
    } else if east && west {
        return '-';
    }

    panic!("Unhandled combination {north} {south} {east} {west}");
}

fn find_to_visit(point: &(usize, usize), rows: &Vec<Vec<char>>, possibilities: &mut Vec<(usize, usize)>) {
    let tile = rows[point.0][point.1];
    match tile {
        '|' => {
            if point.0 > 0 {
                possibilities.push((point.0 - 1, point.1));
            }
            if point.0 < rows.len() - 1 {
                possibilities.push((point.0 + 1, point.1));
            }
        }
        '-' => {
            if point.1 > 0 {
                possibilities.push((point.0, point.1 - 1));
            }
            if point.1 < rows[0].len() - 1 {
                possibilities.push((point.0, point.1 + 1));
            }
        }
        'L' => {
            if point.0 > 0 {
                possibilities.push((point.0 - 1, point.1));
            }
            if point.1 < rows[0].len() - 1 {
                possibilities.push((point.0, point.1 + 1));
            }
        }
        'J' => {
            if point.0 > 0 {
                possibilities.push((point.0 - 1, point.1));
            }
            if point.1 > 0 {
                possibilities.push((point.0, point.1 - 1));
            }
        }
        '7' => {
            if point.0 < rows.len() - 1 {
                possibilities.push((point.0 + 1, point.1));
            }
            if point.1 > 0 {
                possibilities.push((point.0, point.1 - 1));
            }
        }
        'F' => {
            if point.0 < rows.len() - 1 {
                possibilities.push((point.0 + 1, point.1));
            }
            if point.1 < rows[0].len() - 1 {
                possibilities.push((point.0, point.1 + 1));
            }
        }
        _ => { panic!("Unhandled tile {tile}") }
    };
}

fn build_map(path: &str) -> ((usize, usize), Vec<Vec<char>>) {
    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in read_to_string(path).unwrap().lines() {
        let chars = line.chars().collect();
        rows.push(chars);
    }

    let (start, start_tile) = find_start(&rows);
    rows[start.0][start.1] = start_tile;
    println!("start_tile={start_tile}");
    return (start, rows);
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum Dir {
    North,
    South,
    East,
    West
}

pub(crate) fn day10(path: &str) {
    let (start, rows) = build_map(path);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut to_visit: Vec<(usize, usize)> = Vec::new();
    let mut steps = 0_u32;
    let mut map: Vec<Vec<bool>> = Vec::new();

    find_to_visit(&start, &rows, &mut to_visit);

    while !to_visit.is_empty() {
        let mut possibilities: Vec<(usize, usize)> = Vec::new();
        for point in to_visit.iter() {
            visited.insert(*point);
            mark_visited(*point, &mut map);
            find_to_visit(point, &rows, &mut possibilities);
        }
        to_visit.clear();
        possibilities.iter().filter(|x| !visited.contains(*x)).for_each(|x| to_visit.push(*x));
        steps += 1;
    }

    let mut points: Vec<(usize, usize)> = Vec::new();
    let mut current = start;
    let mut dir = if vec!['|', '7', 'J', 'L', 'F'].contains(&rows[start.0][start.1]) {
        North
    } else {
        East
    };

    loop {
        let (next, d) = find_next(current, &dir, &rows);

        if next == start {
            break;
        }
        if dir != d {
            points.push(current);
        }

        current = next;
        dir = d;
    }


    let mut integral = 0i32;
    let mut length = 0i32;
    let mut prev: Option<(usize, usize)> = None;
    points.push(start);
    for p in points {
        if !prev.is_none() {
            let dy = p.1 as i32 - prev.unwrap().1 as i32;
            let dx = p.0 as i32 - prev.unwrap().0 as i32;
            integral += prev.unwrap().0 as i32 * dy;
            length += dy.abs() + dx.abs();
        }
        prev = Some(p);
    }



    let ans = integral.abs() + 1 + length / 2;
    println!("steps={steps} area={ans} integral={}", integral.abs() + 1 - length / 2);
}

fn find_next(c: (usize, usize), d: &Dir, rows: &Vec<Vec<char>>) -> ((usize, usize), Dir) {
    let tile =rows[c.0][c.1];
    match tile {
        '|' => {
            match d {
                North => return ((c.0-1, c.1), North),
                South => return ((c.0+1, c.1), South),
                _ => panic!("error")
            }
        },
        '-' => {
            match d {
                East => return ((c.0, c.1+1), East),
                West => return ((c.0, c.1-1), West),
                _ => panic!("error")
            }
        },
        '7' => {
            match d {
                North => return ((c.0, c.1-1), West),
                East => return ((c.0+1, c.1), South),
                _ => panic!("error")
            }
        },
        'J' => {
            match d {
                South => return ((c.0, c.1 - 1), West),
                East => return ((c.0 - 1, c.1), North),
                _ => panic!("error")
            }
        },
        'L' => {
            match d {
                South => return ((c.0, c.1 + 1), East),
                West => return ((c.0 - 1, c.1), North),
                _ => panic!("error")
            }
        },
        'F' => {
            match d {
                North => return ((c.0, c.1 + 1), East),
                West => return ((c.0 + 1, c.1), South),
                _ => panic!("error")
            }
        },
        _ => {
            panic!("Unsupported!")
        }
    }
}
fn mark_visited(p0: (usize, usize), map: &mut Vec<Vec<bool>>) {
    if p0.0 >= map.len() {
        for _ in 0..=(p0.0 - map.len()) {
            map.push(Vec::new());
        }
    }

    let row= &mut map[p0.0];
    if p0.1 >= row.len() {
        for _ in 0..=(p0.1 - row.len()) {
            row.push(false);
        }
    }

    row[p0.1] = true;
}
