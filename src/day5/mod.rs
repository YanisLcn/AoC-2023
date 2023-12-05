use itertools::Itertools;
use std::u64::{self, MAX};

pub fn part1() -> String {
    let path = "input/day5.txt";
    let input = std::fs::read_to_string(path).unwrap();
    let (v1, mut v2) = input
        .lines()
        .filter(|l| !l.is_empty())
        .fold((Vec::new(), Vec::new()), |(vec1, mut vec2), l| {
            if l.contains(":") && !l.ends_with(":") {
                (
                    Vec::<u64>::new(),
                    l.split(":").collect_vec()[1]
                        .split(" ")
                        .filter(|l| !l.is_empty())
                        .map(|s| {
                            s.parse::<u64>().unwrap()
                        })
                        .collect_vec(),
                )
            } else if l.chars().any(|c| c.is_alphabetic()) {
                vec1.iter().for_each(|&v| (&mut vec2).push(v));
                (vec2, Vec::new())
            } else {
                let line = l
                    .split(" ")
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect_vec();
                let vec1 = vec1
                    .iter()
                    .filter_map(|&val| {
                        let is_contained = line[1] <= val && val <= line[1] + line[2] - 1;
                        if is_contained {
                            (&mut vec2).push(line[0] + val - line[1]);
                            None
                        } else {
                            Some(val)
                        }
                    })
                    .collect_vec();
                (vec1, vec2)
            }
        });
        v1.iter().for_each(|&v| (&mut v2).push(v));
    
        v2.iter()
        .fold(MAX, |min, val| {
            std::cmp::min(min, *val)
        })
        .to_string()
}

pub fn part2() -> String {
    let path = "input/day5.txt";
    let input = std::fs::read_to_string(path).unwrap();
    "".to_string()
}
