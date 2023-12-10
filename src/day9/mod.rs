use itertools::Itertools;

pub fn add_history_forward(mut vec: Vec<i32>) -> i32 {
    let mut total = 0;

    while vec.len() != 1 && vec.iter().all_equal_value() != Ok(&0) {
        total += vec.last().unwrap();
        let mut iterator = vec.iter();
        let previous = iterator.next().unwrap();

        let mut next_vec = Vec::new();
        iterator.fold(previous, |prev, cur| {
            next_vec.push(cur - prev);
            cur
        });
        vec = next_vec;
    }
    total
}
pub fn add_history_backward(mut vec: Vec<i32>) -> i32 {
    let mut total = 0;
    let mut factor = 1;

    while vec.len() != 1 && vec.iter().all_equal_value() != Ok(&0) {
        let to_add = vec.first().unwrap();
        total += factor * to_add;
        factor *= -1;
        let mut iterator = vec.iter();
        let previous = iterator.next().unwrap();

        let mut next_vec = Vec::new();
        iterator.fold(previous, |prev, cur| {
            next_vec.push(cur - prev);
            cur
        });
        vec = next_vec;
    }
    total
}

pub fn part1() -> String {
    let path = "input/day9.txt";
    let input = std::fs::read_to_string(path).unwrap();
    input
        .lines()
        .map(|l| l.split(" ").map(|v| v.parse::<i32>().unwrap()).collect())
        .map(|l| add_history_forward(l))
        .sum::<i32>()
        .to_string()
}

pub fn part2() -> String {
    let path = "input/day9.txt";
    let input = std::fs::read_to_string(path).unwrap();
    input
        .lines()
        .map(|l| l.split(" ").map(|v| v.parse::<i32>().unwrap()).collect())
        .map(|l| add_history_backward(l))
        .sum::<i32>()
        .to_string()
}
