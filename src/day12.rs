use std::collections::HashMap;
use std::fs::read_to_string;

// Copy of https://github.com/jonathanpaulson/AdventOfCode/blob/master/2023/12.py

fn f(springs: &Vec<char>, arrangements: &Vec<u32>, i: usize, bi: usize, current: u32, cache: &mut HashMap<(usize, usize, u32), u64>) -> u64 {
    let key = (i, bi, current);
    let cache_value = cache.get(&key);
    if !cache_value.is_none() {
        return *cache_value.unwrap();
    }
    if i == springs.len() {
        return if bi == arrangements.len() && current == 0 {
            1
        } else if bi == arrangements.len() - 1 && arrangements[bi] == current {
            1
        } else {
            0
        };
    }

    let mut ans = 0;
    for c in vec!['.', '#'] {
        if springs[i] == c || springs[i] == '?' {
            if c == '.' && current == 0 {
                ans += f(springs, arrangements, i + 1, bi, 0, cache);
            } else if c == '.' && current > 0 && bi < arrangements.len() && arrangements[bi] == current {
                ans += f(springs, arrangements, i + 1, bi + 1, 0, cache);
            } else if c == '#' {
                ans += f(springs, arrangements, i + 1, bi, current + 1, cache);
            }
        }
    }

    cache.insert(key, ans);
    return ans;
}

fn process(line: &str, part2: bool) -> u64 {
    let parts: Vec<&str> = line.split(' ').collect();
    let mut springs: Vec<char> = parts[0].chars().collect();
    let mut arrangements: Vec<u32> = parts[1].split(',').map(|x| x.parse::<u32>().unwrap()).collect();

    if part2 {
        springs = [&springs[..], &vec!['?'], &springs[..], &vec!['?'], &springs[..], &vec!['?'], &springs[..], &vec!['?'], &springs[..]].concat();
        arrangements = [&arrangements[..], &arrangements[..], &arrangements[..], &arrangements[..], &arrangements[..]].concat();
    }

    let mut cache: HashMap<(usize, usize, u32), u64> = HashMap::new();
    let ans = f(&springs, &arrangements, 0, 0, 0, &mut cache);

    // println!("{:?} {:?} {ans}", springs.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""), arrangements);

    return ans;
}

pub(crate) fn day12(path: &str, part2: bool) {
    let mut ans = 0;
    for line in read_to_string(path).unwrap().lines() {
        ans += process(line, part2);
    }
    println!("{ans}");
}