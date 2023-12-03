use std::collections::HashMap;
use std::fs::read_to_string;

pub(crate) fn day02_part1(path: &str) {
    let config = HashMap::from([
        ("red", 12), ("green", 13), ("blue", 14)
    ]);

    let mut sum1 = 0;
    let mut sum2 = 0;

    for line in read_to_string(path).unwrap().lines() {
        let parts: Vec<&str> = line.split(":").collect();
        let id = parts[0].split(" ").nth(1).unwrap_or("").parse::<i32>().unwrap();

        let mut min_map: HashMap<&str, i32> = HashMap::new();
        config.keys().for_each(|c| { min_map.insert(c, 0); });

        let mut possible = true;
        for i in parts[1].split(";").collect::<Vec<&str>>() {
            let bags: Vec<&str> = i.split(',').collect();
            for bag in bags {
                let p: Vec<&str> = bag.trim().split(" ").collect();
                let nb = p[0].parse::<i32>().unwrap();
                let color = p[1];

                if nb > config[color] {
                    possible = false;
                }

                min_map.entry(color).and_modify(|v| {
                    if nb > *v {
                        *v = nb;
                    };
                });
            }
        }

        if possible {
            sum1 += id;
        }

        let mut m = 1;
        for v in min_map.values() {
            m *= v;
        }

        sum2 += m;
    }

    println!("part 1 : {:?}", sum1);
    println!("part 2 : {:?}", sum2);
}

