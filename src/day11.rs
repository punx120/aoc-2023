use std::fs::read_to_string;

fn parse_map(path: &str) -> Vec<Vec<char>> {
    let mut temp_rows: Vec<Vec<char>> = Vec::new();
    for line in read_to_string(path).unwrap().lines() {
        let chars = line.chars().collect();
        temp_rows.push(chars);
    }
    temp_rows
}

pub(crate) fn day11(path: &str, expansion: u64) {
    let rows = parse_map(path);

    let mut row_mapping: Vec<u64> = (0..(rows.len() as u64)).collect();
    let mut col_mapping: Vec<u64> = (0..(rows.len() as u64)).collect();

    for i in 0..rows.len() {
        if rows[i].iter().all(|x| *x == '.') {
            for j in (i + 1)..row_mapping.len() {
                row_mapping[j] += expansion;
            }
        }
    }

    for i in 0..rows[0].len() {
        if rows.iter().all(|x| (*x)[i] == '.') {
            for j in (i + 1)..col_mapping.len() {
                col_mapping[j] += expansion;
            }
        }
    }

    let mut pp: Vec<(usize, usize)> = Vec::new();
    let mut positions: Vec<(u64, u64)> = Vec::new();

    for r in 0..rows.len() {
        for c in 0..rows[r].len() {
            if rows[r][c] == '#' {
                pp.push((r, c));
                positions.push((row_mapping[r], col_mapping[c]));
            }
        }
    }

    let mut total = 0;
    for p in 0..positions.len() {
        let position = positions[p];
        for o in (p + 1)..positions.len() {
            let other = positions[o];
            let distance = distance(position, other);
            total += distance;
        }
    }

    println!("{total}");
}

fn distance(a: (u64, u64), b: (u64, u64)) -> u64 {
    let r = a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
    r
}