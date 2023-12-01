pub fn part1() -> String {
    let path = "input/day1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    format!("{}",input.lines().map(|l| first_and_last_digits(l)).sum::<u32>())
}

fn first_and_last_digits(line: &str) -> u32 {
    let rev_line = line.chars().rev().collect::<String>();
    let fst: usize = line.find(|c: char| c.is_numeric()).unwrap();
    let lst: usize = rev_line.find(|c: char| c.is_numeric()).unwrap();
    format!(
        "{}{}",
        line.get(fst..fst + 1).unwrap(),
        rev_line.get(lst..lst + 1).unwrap()
    )
    .to_string()
    .parse::<u32>()
    .unwrap()
}

pub fn part2() -> String {
    let path = "input/day1.txt";
    let input = std::fs::read_to_string(path).unwrap();
    "".to_string()
}
