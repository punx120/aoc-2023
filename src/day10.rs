use std::collections::HashSet;
use std::fs::read_to_string;

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

pub(crate) fn day10(path: &str) {
    let (start, rows) = build_map(path);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut to_visit: Vec<(usize, usize)> = Vec::new();
    let mut steps = 0_u32;

    find_to_visit(&start, &rows, &mut to_visit);

    while !to_visit.is_empty() {
        let mut possibilities: Vec<(usize, usize)> = Vec::new();
        for point in to_visit.iter() {
            visited.insert(*point);
            find_to_visit(point, &rows, &mut possibilities);
        }
        to_visit.clear();
        possibilities.iter().filter(|x| !visited.contains(*x)).for_each(|x| to_visit.push(*x));
        steps += 1;
    }

    println!("steps={steps}")
}

pub(crate) fn day10_part2(path: &str) {
    let (start, rows) = build_map(path);

}