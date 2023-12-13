use std::cmp::max;
use std::fs::read_to_string;

fn check_horizontal(i: usize, lines: &Vec<Vec<char>>, smudge: bool) -> bool {
    let mut l = i - 1;
    let mut r = i;
    let mut modified = false;
    while r < lines[0].len() {
        if !same_columns(lines, l, r, smudge, &mut modified) {
            return false;
        }

        if l == 0 {
            break;
        }

        l -= 1;
        r += 1;
    }
    return true;
}

fn same_columns(lines: &Vec<Vec<char>>, c1: usize, c2: usize, smudge: bool, modified: &mut bool) -> bool {
    for i in 0..lines.len() {
        if lines[i][c1] != lines[i][c2] {
            if !smudge || *modified {
                return false;
            }
            *modified = true;
        }
    }
    return true;
}

fn same_row(lines: &Vec<Vec<char>>, r1: usize, r2: usize, smudge: bool, modified: &mut bool) -> bool {
    for c in 0..lines[0].len() {
        if lines[r1][c] != lines[r2][c] {
            if !smudge || *modified {
                return false;
            }
            *modified = true;
        }
    }
    return true;
}

fn check_vertical(i: usize, lines: &Vec<Vec<char>>, smudge: bool) -> bool {
    let mut t = i - 1;
    let mut b = i;
    let mut modified = false;
    while b < lines.len() {
        if !same_row(lines, t, b, smudge, &mut modified) {
            return false;
        }
        if t == 0 {
            break;
        }
        t -= 1;
        b += 1;
    }

    return true;
}

fn process(lines: &Vec<Vec<char>>, smudge: bool, previous: u32) -> u32 {
    let rows = lines.len();
    let columns = lines[0].len();

    for i in 1..max(rows, columns) {
        if i < rows && check_vertical(i, lines, smudge) {
            println!("smudge={smudge} vertical @ {i}");
            let ans = (i * 100) as u32;
            if ans != previous {
                return ans;
            }
        }

        if i < columns && check_horizontal(i, lines, smudge) {
            println!("smudge={smudge} horizontal @ {i}");
            let ans = i as u32;
            if ans != previous {
                return ans;
            }
        }
    }

    panic!("No mirror found ...");
}

pub(crate) fn day13(path: &str) {
    let mut ans = 0;
    let mut ans_smudge = 0;
    let mut lines: Vec<Vec<char>> = Vec::new();
    for line in read_to_string(path).unwrap().lines() {
        if line.is_empty() {
            let regular = process(&lines, false, 0);
            ans += regular;
            let smudge = process(&lines, true, regular);
            if regular == smudge {
                panic!("No smudge");
            }

            ans_smudge += smudge;
            lines.clear();
        } else {
            lines.push(line.chars().collect());
        }
    }

    ans += process(&lines, false, 0);
    ans_smudge += process(&lines, true, ans);

    println!("{ans} {ans_smudge}");
}