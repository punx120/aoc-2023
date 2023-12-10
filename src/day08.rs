use std::collections::HashMap;
use std::fs::read_to_string;

pub(crate) fn day08(path: &str, multiple: bool) {
    let mut map: HashMap<&str, (String, String)> = HashMap::new();
    let mut direction: Option<&str> = None;
    let lines = read_to_string(path).unwrap();
    for line in lines.lines() {
        if direction.is_none() {
            direction = Some(line);
        } else if line.trim().len() > 0 {
            let row: Vec<&str> = line.split('=').collect();
            let clean = row[1].replace('(', "").replace(")", "");
            let e: Vec<&str> = clean.split(',').collect();
            map.insert(row[0].trim(), (e[0].trim().to_string(), e[1].trim().to_string()));
        }
    }

    if !multiple {
        let mut i: usize = 0;
        let mut location = "AAA";
        let dir = direction.unwrap();
        let dir_len = dir.len();
        while location != "ZZZ" {
            let d = dir.chars().nth(i % dir_len).unwrap();
            match d {
                'R' => { location = &map[&location].1; }
                'L' => { location = &map[&location].0; }
                _ => { panic!("Error") }
            }
            i += 1;
        }
        println!("{i}")
    } else {
        let mut locations: Vec<&str> = Vec::new();
        for key in map.keys() {
            if key.chars().nth(2) == Some('A') {
                locations.push(key);
            }
        }

        println!("{:?}", locations);

        let mut times : Vec::<usize> = Vec::new();

        for location in locations {
            let mut loc = location;

            let mut i: usize = 0;
            let dir = direction.unwrap();
            let dir_len = dir.len();

            while true {
                let d = dir.chars().nth(i % dir_len).unwrap();
                match d {
                    'R' => { loc = &map[loc].1; }
                    'L' => { loc = &map[loc].0; }
                    _ => { panic!("Error") }
                }
                i += 1;
                if loc.chars().nth(2) == Some('Z') {
                    times.push(i);
                    break
                }
            }
        }
        let mut l: u64 = 1;
        for time in times {
            l = num_integer::lcm(l, time as u64);
        }
        println!("{:?}", l);
    }
}

fn all_done(locations: &Vec<&str>) -> bool {
    for location in locations {
        if location.chars().nth(2) != Some('Z') {
            return false;
        }
    }
    return true;
}