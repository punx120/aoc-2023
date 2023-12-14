use std::fs::read_to_string;

fn adjust(pos: usize, dir: i8) -> usize {
    if dir > 0 {
        return pos + 1;
    } else if pos > 0 {
        return pos - 1;
    } else {
        return pos;
    }
}

fn roll_up_down(map: &mut Vec<Vec<char>>, north: bool) {
    let range: Vec<usize>;
    if north {
        range = (0..map.len()).collect();
    } else {
        range = (0..map.len()).rev().collect();
    }

    let direction: i8 = if north { 1 } else { -1 };
    for c in 0..map[0].len() {
        let mut index_to_move: Option<usize> = None;
        for r in range.iter() {
            if map[*r][c] == '#' {
                index_to_move = Some(adjust(*r, direction));
            } else if map[*r][c] == '.' {
                if index_to_move.is_none() {
                    index_to_move = Some(*r);
                }
            } else if map[*r][c] == 'O' {
                if !index_to_move.is_none() {
                    let dest = index_to_move.unwrap();
                    index_to_move = Some(adjust(dest, direction));
                    if dest != *r {
                        let _ = std::mem::replace(&mut map[dest][c], 'O');
                        let _ = std::mem::replace(&mut map[*r][c], '.');
                    }
                }
            }
        }
    }
}


fn roll_left_right(map: &mut Vec<Vec<char>>, left: bool) {
    let range: Vec<usize>;
    if left {
        range = (0..map[0].len()).collect();
    } else {
        range = (0..map[0].len()).rev().collect();
    }

    let direction: i8 = if left { 1 } else { -1 };
    for r in 0..map.len() {
        let mut index_to_move: Option<usize> = None;
        for c in range.iter() {
            if map[r][*c] == '#' {
                index_to_move = Some(adjust(*c, direction));
            } else if map[r][*c] == '.' {
                if index_to_move.is_none() {
                    index_to_move = Some(*c);
                }
            } else if map[r][*c] == 'O' {
                if !index_to_move.is_none() {
                    let dest = index_to_move.unwrap();
                    index_to_move = Some(adjust(dest, direction));
                    if dest != *c {
                        let _ = std::mem::replace(&mut map[r][dest], 'O');
                        let _ = std::mem::replace(&mut map[r][*c], '.');
                    }
                }
            }
        }
    }
}

fn weight(map: &Vec<Vec<char>>) -> usize {
    let mut sum: usize = 0;
    let height = map.len();
    for c in 0..map[0].len() {
        for r in 0..map.len() {
            if map[r][c] == 'O' {
                sum += height - r;
            }
        }
    }
    return sum;
}

pub(crate) fn day14(path: &str) {
    let mut map = read_map(path);
    roll_up_down(&mut map, true);
    println!("{}", weight(&map));
}

pub(crate) fn day14_part2(path: &str) {
    let mut map = read_map(path);

    for _ in 0..100 {
        roll_up_down(&mut map, true);
        roll_left_right(&mut map, true);
        roll_up_down(&mut map, false);
        roll_left_right(&mut map, false);
    }

    let mut cycle : Vec<usize> = Vec::new();

    for i in 0..100 {
        roll_up_down(&mut map, true);
        roll_left_right(&mut map, true);
        roll_up_down(&mut map, false);
        roll_left_right(&mut map, false);

        cycle.push(weight(&map));
    }

    let mut cycle_len = None;
    for end in 2..cycle.len() {
        if !cycle_len.is_none() {
            break;
        }

        let pattern = &cycle[0..end];
        for start in (end..cycle.len()).step_by(end) {
            if start+end >= cycle.len() {
                cycle_len = Some(end);
                break;
            }
            let temp = &cycle[start..start+end];
            if temp != pattern {
                break;
            }
        }
    }

    if cycle_len.is_none() {
        panic!("No cycle found...");
    } else {
        for i in 0..cycle_len.unwrap() {
            let x = (999_999_999 - (100 + i)) % cycle_len.unwrap();
            if x == 0 {
                println!("{}", cycle[i]);
            }
        }
    }
}

fn read_map(path: &str) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in read_to_string(path).unwrap().lines() {
        map.push(line.chars().collect());
    }
    map
}
