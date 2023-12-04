use std::usize;

use itertools::Itertools;

pub fn part1() -> String {
    let path = "input/day4.txt";
    let input = std::fs::read_to_string(path).unwrap();
    input
        .lines()
        .map(|l| l.split(": ").collect_tuple::<(&str, &str)>().unwrap())
        .map(|(_, numbers)| {
            numbers
                .split(" | ")
                .collect_tuple::<(&str, &str)>()
                .unwrap()
        })
        .map(|(winners, values)| {
            (
                winners
                    .split(" ")
                    .filter_map(|v| v.parse::<u32>().ok())
                    .collect::<Vec<u32>>(),
                values
                    .split(" ")
                    .filter_map(|v| v.parse::<u32>().ok())
                    .collect::<Vec<u32>>(),
            )
        })
        .filter_map(|(w, v)| {
            let total = v
                .iter()
                .fold(0, |t, val| t + if w.contains(val) { 1 } else { 0 });
            if total > 0 {
                Some(total - 1)
            } else {
                None
            }
        })
        .map(|v| 2_u32.pow(v))
        .sum::<u32>()
        .to_string()
}

pub fn part2() -> String {
    let path = "input/day4.txt";
    let input = std::fs::read_to_string(path).unwrap();
    let scratchpads: &mut Vec<u32> = &mut Vec::new();
    input
        .lines()
        .map(|l| l.split(": ").collect_tuple::<(&str, &str)>().unwrap())
        .map(|(_, numbers)| {
            numbers
                .split(" | ")
                .collect_tuple::<(&str, &str)>()
                .unwrap()
        })
        .map(|(winners, values)| {
            (
                winners
                    .split(" ")
                    .filter_map(|v| v.parse::<u32>().ok())
                    .collect::<Vec<u32>>(),
                values
                    .split(" ")
                    .filter_map(|v| v.parse::<u32>().ok())
                    .collect::<Vec<u32>>(),
            )
        })
        .map(|(w, v)| -> u32 {
            v.iter()
                .fold(0_u32, |t, val| if w.contains(val) { t + 1 } else { t })
        })
        .fold(0, |total_pads, total_winners| -> u32 {
            let actual = if scratchpads.len() == 0 {
                1
            } else {
                1 + scratchpads.remove(0)
            };
            let len: u32 = scratchpads.len().try_into().unwrap();
            for index in 0..total_winners {
                if index < len {
                    scratchpads[index as usize] += actual;
                } else {
                    scratchpads.push(actual);
                }
            }
            total_pads + actual
        })
        .to_string()
}
