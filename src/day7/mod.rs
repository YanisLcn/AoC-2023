use std::{cmp::Ordering, collections::HashMap, iter::zip, u64};

use itertools::Itertools;

#[derive(Debug)]
pub enum Hands {
    FiveOfAKind(String),
    FourOfAKind(String),
    FullHouse(String),
    ThreeOfAKind(String),
    TwoPair(String),
    OnePair(String),
    HighCard(String),
}

pub fn val_of_hand(hand: &Hands) -> u64 {
    match hand {
        Hands::FiveOfAKind(_) => 1,
        Hands::FourOfAKind(_) => 2,
        Hands::FullHouse(_) => 3,
        Hands::ThreeOfAKind(_) => 4,
        Hands::TwoPair(_) => 5,
        Hands::OnePair(_) => 6,
        Hands::HighCard(_) => 7,
    }
}

pub fn str_of_hand(hand: &Hands) -> String {
    match hand {
        Hands::FiveOfAKind(s)
        | Hands::FourOfAKind(s)
        | Hands::FullHouse(s)
        | Hands::ThreeOfAKind(s)
        | Hands::TwoPair(s)
        | Hands::OnePair(s)
        | Hands::HighCard(s) => s.to_string(),
    }
}

pub fn val_of_char(c: char) -> u64 {
    match c {
        'A' => 1,
        'K' => 2,
        'Q' => 3,
        'J' => 16,
        'T' => 5,
        c if c.is_digit(10) => 15 - c.to_digit(10).unwrap() as u64,
        _ => unreachable!(),
    }
}

impl PartialOrd for Hands {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hands {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_value = val_of_hand(self);
        let other_value = val_of_hand(other);
        if self_value < other_value {
            Ordering::Less
        } else if self_value > other_value {
            Ordering::Greater
        } else {
            zip(str_of_hand(self).chars(), str_of_hand(other).chars()).fold(
                Ordering::Equal,
                |order, (a, b)| match order {
                    Ordering::Equal => val_of_char(a).cmp(&val_of_char(b)),
                    ordering => ordering,
                },
            )
        }
    }
}

impl PartialEq for Hands {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Hands {}

pub fn hand_of_str(hand: &str) -> Hands {
    let sets = hand
        .chars()
        .fold(HashMap::new(), |mut d: HashMap<char, u64>, c| {
            let key = d.get_mut(&c);
            if key == None {
                d.insert(c, 1);
            } else {
                *key.unwrap() += 1;
            };
            d
        });

    match sets.keys().len() {
        1 => Hands::FiveOfAKind(hand.to_string()),
        2 => {
            if sets.keys().contains(&'J') {
                return Hands::FiveOfAKind(hand.to_string());
            }
            let len = sets.values().next().unwrap();
            if *len == 1 || *len == 4 {
                Hands::FourOfAKind(hand.to_string())
            } else {
                Hands::FullHouse(hand.to_string())
            }
        }
        3 => match sets.get(&'J') {
            None => {
                let mut values = sets.values();
                let len_1 = values.next().unwrap();
                let len_2 = values.next().unwrap();
                if *len_1 == 2 || *len_2 == 2 {
                    Hands::TwoPair(hand.to_string())
                } else {
                    Hands::ThreeOfAKind(hand.to_string())
                }
            }
            Some(v) => match v {
                1 => {
                    let mut values = sets.values();
                    let len_1 = values.next().unwrap();
                    let len_2 = values.next().unwrap();
                    if *len_1 == 2 || *len_2 == 2 {
                        Hands::FullHouse(hand.to_string())
                    } else {
                        Hands::FourOfAKind(hand.to_string())
                    }
                }
                2 | 3 => Hands::FourOfAKind(hand.to_string()),
                _ => unreachable!(),
            },
        },
        4 => match sets.get(&'J') {
            Some(v) => Hands::ThreeOfAKind(hand.to_string()),
            None => Hands::OnePair(hand.to_string()),
        },
        5 => match sets.get(&'J') {
            Some(_) => Hands::OnePair(hand.to_string()),
            None => Hands::HighCard(hand.to_string()),
        },
        _ => unreachable!(),
    }
}

pub fn part1() -> String {
    let path = "input/day7.txt";
    let input = std::fs::read_to_string(path).unwrap();
    let mut lines: Vec<(u64, Hands)> = input
        .lines()
        .map(|line| line.split(" ").collect())
        .map(|v: Vec<&str>| (v[1].parse::<u64>().unwrap(), hand_of_str(v[0])))
        .collect();
    lines.sort_by(|(_, hand_a), (_, hand_b)| hand_a.cmp(&hand_b));
    lines
        .iter()
        .map(|el| el.0)
        .rev()
        .enumerate()
        .map(|(index, value)| (index as u64 + 1) * value)
        .sum::<u64>()
        .to_string()
}

pub fn part2() -> String {
    let path = "input/day7.txt";
    let input = std::fs::read_to_string(path).unwrap();
    let mut lines: Vec<(u64, Hands)> = input
        .lines()
        .map(|line| line.split(" ").collect())
        .map(|v: Vec<&str>| (v[1].parse::<u64>().unwrap(), hand_of_str(v[0])))
        .collect();
    lines.sort_by(|(_, hand_a), (_, hand_b)| hand_a.cmp(&hand_b));
    lines
        .iter()
        .map(|el| el.0)
        .rev()
        .enumerate()
        .map(|(index, value)| (index as u64 + 1) * value)
        .sum::<u64>()
        .to_string()
}
