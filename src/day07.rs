use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u32,
    rank: u32,
}

fn build_optimized(cards: String, bid: u32, jokers: u32, map: HashMap<char, i32>) -> Hand {
    let mut nb_three = 0;
    let mut nb_pair = 0;
    let mut nb_four = 0;
    let mut rank = 0;

    for value in map.into_values() {
        match value {
            5 => {
                rank = 1;
                break;
            }
            4 => {
                nb_four += 1;
            }
            3 => { nb_three += 1 }
            2 => { nb_pair += 1 }
            _ => {}
        }
    }

    if rank == 0 {
        if nb_four == 1 {
            if jokers == 1 {
                rank = 1;
            } else {
                panic!("Should not come here")
            }
        } else if nb_three == 1 {
            if jokers == 1 {
                rank = 2;
            } else if jokers == 2 {
                rank = 1;
            }
        } else if nb_pair == 2 {
            if jokers == 1 {
                rank = 3
            } else {
                panic!("Should not come here")
            }
        } else if nb_pair == 1 {
            if jokers == 1 {
                rank = 4
            } else if jokers == 2 {
                rank = 2
            } else if jokers == 3 {
                rank = 1
            }
        } else {
            if jokers == 1 {
                rank = 6;
            } else if jokers == 2 {
                rank = 4;
            } else if jokers == 3 {
                rank = 2;
            } else if jokers == 4 {
                rank = 1;
            } else if jokers == 5 {
                rank = 1;
            }
        }
    }

    if rank == 0 {
        panic!("Should not come here")
    }

    Hand { cards, bid, rank }
}

fn build_hand(cards: String, bid: u32, map: HashMap<char, i32>) -> Hand {
    let mut nb_three = 0;
    let mut nb_pair = 0;
    let mut rank = 0;

    for value in map.into_values() {
        match value {
            5 => {
                rank = 1;
                break;
            }
            4 => {
                rank = 2;
                break;
            }
            3 => { nb_three += 1 }
            2 => { nb_pair += 1 }
            _ => {}
        }
    }

    if rank == 0 {
        if nb_three == 1 {
            if nb_pair == 1 {
                rank = 3;
            } else {
                rank = 4;
            }
        } else if nb_pair == 2 {
            rank = 5;
        } else if nb_pair == 1 {
            rank = 6;
        } else {
            rank = 7;
        }
    }

    Hand { cards, bid, rank }
}

impl Hand {
    pub fn new(cards: String, bid: u32, joker: bool) -> Self {
        let mut map = HashMap::new();
        let mut jokers = 0;
        for c in cards.to_string().chars() {
            if c == 'J' {
                jokers += 1;
            } else {
                *map.entry(c).or_insert(0) += 1;
            }
        }

        if !joker || jokers == 0 {
            build_hand(cards, bid, map)
        } else {
            build_optimized(cards, bid, jokers, map)
        }
    }
}

pub(crate) fn day07(path: &str) {
    let mut hands: Vec<Hand> = Vec::new();

    for line in read_to_string(path).unwrap().lines() {
        let l: Vec<&str> = line.split(' ').collect();
        hands.push(Hand::new(
            l.get(0).unwrap().to_string(),
            l.get(1).unwrap().trim().parse::<u32>().unwrap(),
            false,
        ));
    }

    hands.sort_by(|a, b| {
        if a.rank != b.rank {
            b.rank.cmp(&a.rank)
        } else {
            for i in 0..5 {
                let x = a.cards.chars().nth(i).unwrap();
                let y = b.cards.chars().nth(i).unwrap();
                if x != y {
                    let card_rank = HashMap::from([('A', 1), ('K', 2), ('Q', 3), ('J', 4), ('T', 5), ('9', 6), ('8', 7), ('7', 8), ('6', 9), ('5', 10), ('4', 11), ('3', 12), ('2', 13)]);
                    return card_rank[&y].cmp(&card_rank[&x]);
                }
            }
            return Ordering::Equal;
        }
    });

    // println!("{:?}", hands);
    let mut ans = 0;

    for i in 0..hands.len() {
        ans += (i as u32 + 1) * hands[i].bid;
    }

    println!("{ans}");
}

pub(crate) fn day07_part2(path: &str) {
    let mut hands: Vec<Hand> = Vec::new();

    for line in read_to_string(path).unwrap().lines() {
        let l: Vec<&str> = line.split(' ').collect();
        hands.push(Hand::new(
            l.get(0).unwrap().to_string(),
            l.get(1).unwrap().trim().parse::<u32>().unwrap(),
            true));
    }

    hands.sort_by(|a, b| {
        if a.rank != b.rank {
            b.rank.cmp(&a.rank)
        } else {
            for i in 0..5 {
                let x = a.cards.chars().nth(i).unwrap();
                let y = b.cards.chars().nth(i).unwrap();
                if x != y {
                    let card_rank = HashMap::from([('A', 1), ('K', 2), ('Q', 3), ('T', 5), ('9', 6), ('8', 7), ('7', 8), ('6', 9), ('5', 10), ('4', 11), ('3', 12), ('2', 13), ('J', 14), ]);
                    return card_rank[&y].cmp(&card_rank[&x]);
                }
            }
            return Ordering::Equal;
        }
    });

    let mut ans = 0;

    for i in 0..hands.len() {
        ans += (i as u32 + 1) * hands[i].bid;
    }

    println!("{ans}");
}