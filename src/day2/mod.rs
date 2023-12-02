use std::cmp::max;

use itertools::Itertools;

const MAX_COLORS: [(u32, &'static str); 3] = [(12, "red"), (13, "green"), (14, "blue")];

fn get_id(game: &str) -> u32 {
    game.split(" ")
        .skip(1)
        .collect::<Vec<&str>>()
        .pop()
        .unwrap()
        .parse::<u32>()
        .unwrap()
}

fn extract_colors(colors: &str) -> Vec<Vec<(u32, &str)>> {
    colors
        .split("; ")
        .map(|part: &str| {
            part.split(", ")
                .map(|clr: &str| clr.split(" ").collect_tuple::<(&str, &str)>().unwrap())
                .map(|(nb, clr): (&str, &str)| (nb.parse::<u32>().unwrap(), clr))
                .collect::<Vec<(u32, &str)>>()
        })
        .collect::<Vec<Vec<(u32, &str)>>>()
}

pub fn part1() -> String {
    let path = "input/day2.txt";
    let input = std::fs::read_to_string(path).unwrap();
    format!(
        "{}",
        input
            .lines()
            .map(|l| l.split(": ").collect_tuple::<(&str, &str)>().unwrap())
            .map(|(id, clrs)| (get_id(id), extract_colors(clrs)))
            .filter(|(_, clrs)| {
                clrs.iter().all(|part| {
                    part.iter().all(|(nb, clr)| {
                        MAX_COLORS.iter().find(|(max_nb, c)| clr == c).unwrap().0 >= *nb
                    })
                })
            })
            .map(|(id, _)| id)
            .sum::<u32>()
    )
}

pub fn part2() -> String {
    let path = "input/day2.txt";
    let input = std::fs::read_to_string(path).unwrap();

    format!(
        "{}",
        input
            .lines()
            .map(|l| l.split(": ").collect_tuple::<(&str, &str)>().unwrap())
            .map(|(_, clrs)| extract_colors(clrs))
            .fold(0, |token, clrs| {
                clrs.iter()
                    .fold(
                        vec![(0, "red"), (0, "blue"), (0, "green")],
                        |colors, part| {
                            part.iter().fold(colors, |part_colors, (nb, color)| {
                                part_colors
                                    .iter()
                                    .map(|(total, c)| {
                                        (if c == color { *max(total, nb) } else { *total }, *c)
                                    })
                                    .collect()
                            })
                        },
                    )
                    .iter()
                    .fold(1, |acc, (t, _)| acc * t)
                    + token
            })
    )
}
