use std::{iter::zip, u64};

use itertools::Itertools;

pub fn part1() -> String {
    let path = "input/day6.txt";
    let input = std::fs::read_to_string(path).unwrap();
    let vecs: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.split(":")
                .last()
                .unwrap()
                .split(" ")
                .filter_map(|e| {
                    if e.is_empty() {
                        None
                    } else {
                        Some(e.parse::<u32>().unwrap())
                    }
                })
                .collect_vec()
        })
        .collect();
    let time = &vecs[0];
    let dists = &vecs[1];

    assert_eq!(time.len(), dists.len());
    let time_dist = zip(time, dists);
    time_dist
        .map(|(t, d)| {
            (1..*t).into_iter().fold(0, |counter, k| {
                counter + if k * (*t - k) > *d { 1 } else { 0 }
            })
        })
        .product::<u32>()
        .to_string()
}

pub fn part2() -> String {
    let path = "input/day6.txt";
    let input = std::fs::read_to_string(path).unwrap();
    let vecs: Vec<u64> = input
        .lines()
        .map(|l| {
            l.split(":")
                .last()
                .unwrap()
                .replacen(" ", "", usize::MAX)
                .parse::<u64>().unwrap()
                
        })
        .collect();
    let time = &vecs[0];
    let dists = &vecs[1];

    (1..*time).into_iter().fold(0, |counter, k| -> u64 {
        counter + if k * (*time - k) > *dists { 1 } else { 0 }
    })
    .to_string()
}
