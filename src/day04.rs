use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::fs::read_to_string;

pub(crate) fn day04(path: &str) {
    let mut sum: u32 = 0;
    let mut card_count: HashMap<usize, u32> = HashMap::new();
    card_count.insert(1, 1);

    for line in read_to_string(path).unwrap().lines() {
        let card = line.split(':').collect::<Vec<&str>>();
        let number = card[0].split(' ').last().unwrap().trim().parse::<usize>().unwrap();

        let nb_card = if card_count.contains_key(&number) { card_count[&number] } else { 1 };

        let split: Vec<&str> = card[1].split('|').collect();
        let winning: Vec<u32> = split[0].split(' ').filter(|x| !x.trim().is_empty())
            .map(|x| x.trim().parse::<u32>().unwrap()).collect();

        let winning_set: HashSet<&u32> = HashSet::from_iter(winning.iter());

        let count = split[1].split(' ').filter(|x| !x.trim().is_empty())
            .map(|x| x.trim().parse::<u32>().unwrap())
            .filter(|x| winning_set.contains(x)).collect::<Vec<u32>>().len();
        if count > 0 {
            card_count.entry(number).or_insert(1);

            for i in (number + 1)..(number + count + 1) {
                let values = match card_count.entry(i) {
                    Entry::Occupied(o) => o.into_mut(),
                    Entry::Vacant(v) => v.insert(1),
                };
                *values = *values + nb_card;
            }
            sum += 2_u32.pow((count - 1) as u32);
        } else {
            card_count.entry(number).or_insert(1);
        }
    }

    println!("{:?}", sum);
    println!("{:?}", card_count.values().sum::<u32>());
}