use std::fs::read_to_string;

pub(crate) fn day05(path: &str) {
    let mut seeds: Vec<u32> = Vec::new();
    let mut to_process: Vec<usize> = Vec::new();

    for line in read_to_string(path).unwrap().lines() {
        if line.starts_with("seeds:") {
            seeds = line.split(':').last().unwrap().split(' ')
                .filter(|x| !x.trim().is_empty())
                .map(|x| x.trim().parse::<u32>().unwrap()).collect();
            to_process = (0..seeds.len()).collect();
            continue;
        }

        if !line.is_empty() {
            if !line.contains("map") {
                let range: Vec<u32> = line.trim().split(' ')
                    .map(|x| x.trim().parse::<u32>().unwrap()).collect();
                to_process.retain(|i| {
                    if seeds[*i] >= range[1] && seeds[*i] <= range[1] + range[2] {
                        seeds[*i] = range[0] + (seeds[*i] - range[1]);
                        return false;
                    }
                    return true;
                });
            }
        } else {
            to_process = (0..seeds.len()).collect();
        }
    }

    println!("{:?}", seeds.iter().min().unwrap());
}

pub(crate) fn day05_part2(path: &str) {
    let mut seeds: Vec<(u64, u64)> = Vec::new();
    let mut modified_seeds: Vec<(u64, u64)> = Vec::new();

    for line in read_to_string(path).unwrap().lines() {
        if line.starts_with("seeds:") {
            let seed_range: Vec<u64> = line.split(':').last().unwrap().split(' ')
                .filter(|x| !x.trim().is_empty())
                .map(|x| x.trim().parse::<u64>().unwrap()).collect();

            for i in (0..seed_range.len()).step_by(2) {
                seeds.push((seed_range[i], seed_range[i] + seed_range[i + 1] - 1));
            }
            continue;
        }


        if !line.is_empty() {
            if !line.contains("map") {
                let range: Vec<u64> = line.trim().split(' ')
                    .map(|x| x.trim().parse::<u64>().unwrap()).collect();

                let ts = range[1];
                let te = range[1] + range[2];

                let mut unprocessed_seeds: Vec<(u64, u64)> = Vec::new();

                for (s, e) in seeds.iter_mut() {
                    if ts <= *s && *e <= te { // (s,e) is within (ts,te) - move the whole range
                        *s = range[0] + (*s - ts);
                        *e = range[0] + (*e - ts);
                        modified_seeds.push((*s, *e));
                    } else if *s <= ts && te <= *e { // (ts,te) is within (s,e)
                        unprocessed_seeds.push((*s, ts - 1));
                        unprocessed_seeds.push((te + 1, *e));
                        let ns = range[0];
                        let ne = range[0] + range[2];
                        modified_seeds.push((ns, ne));
                    } else if ts < *s && *s <= te {
                        let ns = range[0] + (*s - ts);
                        let ne = range[0] + range[2];
                        modified_seeds.push((ns, ne));
                        unprocessed_seeds.push((te + 1, *e));
                    } else if ts <= *e && *e < te {
                        let ns = range[0];
                        let ne = range[0] + (*e - ts);
                        modified_seeds.push((ns, ne));
                        unprocessed_seeds.push((*s, ts - 1));
                    } else {
                        if te < *s || ts > *e { // no match - ok
                            unprocessed_seeds.push((*s, *e));
                        } else {
                            println!("unhandled : {s}..{e} {ts}..{te}");
                        }
                    }
                }

                seeds = unprocessed_seeds;
            } else {
                // println!("{:?}", seeds);
                // println!("{:?}", line);
            }
        } else {
            if modified_seeds.len() > 0 {
                seeds.append(&mut modified_seeds);
                // seeds = std::mem::take(&mut modified_seeds);
                // println!("{:?}", seeds);
            }
        }
    }

    if modified_seeds.len() > 0 {
        seeds.append(&mut modified_seeds);
    }

    // println!("{:?}", seeds);
    println!("{:?}", seeds.iter().map(|(s,e)| s).min().unwrap());
}
