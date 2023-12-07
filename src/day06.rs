use std::fs::read_to_string;

pub(crate) fn day06(path: &str) {
    let binding = read_to_string(path).unwrap();
    let lines: Vec<&str> = binding.lines().collect();
    let times: Vec<u32> = lines[0].split(':').nth(1).unwrap().split(' ')
        .filter(|x| !x.trim().is_empty())
        .map(|x| x.trim().parse::<u32>().unwrap()).collect();
    let distances: Vec<u32> = lines[1].split(':').nth(1).unwrap().split(' ')
        .filter(|x| !x.trim().is_empty())
        .map(|x| x.trim().parse::<u32>().unwrap()).collect();

    println!("{:?}", times);
    println!("{:?}", distances);

    let mut res = 1;

    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];
        let mut ways = 0;

        for t in 1..time {
            let remaining = time - t;
            let travel = t * remaining;
            if travel > distance {
                ways += 1;
            }
        }

        println!("{i} {ways} ways");
        res *= ways;
    }

    println!("{res}");
}

pub(crate) fn day06_part2(path: &str) {
    let binding = read_to_string(path).unwrap();
    let lines: Vec<&str> = binding.lines().collect();
    let time = lines[0].split(':').nth(1).unwrap().replace(' ', "").trim().parse::<u64>().unwrap();
    let distance = lines[1].split(':').nth(1).unwrap().replace(' ', "").trim().parse::<u64>().unwrap();

    println!("{:?}", time);
    println!("{:?}", distance);

    let mut ways = 0;

    for t in 1..time {
        let remaining = time - t;
        let travel = t * remaining;
        if travel > distance {
            ways += 1;
        } else if ways > 0 {
            break;
        }
    }

    println!("{ways}")
}