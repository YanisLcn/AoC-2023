use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug)]
pub enum Instruction {
    Left,
    Right,
}

pub fn part1() -> String {
    let path = "input/day8.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let mut lines = input.lines();
    let instructions: Vec<Instruction> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => unreachable!(),
        })
        .collect::<Vec<Instruction>>();

    let mut first = false;
    let mut key: &str = &"";
    let mut hashmap = HashMap::new();
    lines
        .filter(|l| !l.is_empty())
        .map(|s| s.split(" = ").collect())
        .map(|lines: Vec<&str>| {
            if !first {
                key = &lines[0];
                first = true;
            }
            (lines[0], lines[1].split(", ").collect::<Vec<&str>>())
        })
        .for_each(|(key, values)| {
            hashmap.insert(
                key,
                (values[0].replace("(", ""), values[1].replace(")", "")),
            );
        });

    let mut index = 0;

    let mut instr_iterator = instructions.iter();
    key = "AAA";
    while key.cmp("ZZZ") != Ordering::Equal {
        let current_instr = match instr_iterator.next() {
            Some(instr) => instr,
            None => {
                instr_iterator = instructions.iter();
                instr_iterator.next().unwrap()
            }
        };
        index += 1;
        let ways = hashmap.get(key).unwrap();
        key = match current_instr {
            Instruction::Left => &ways.0,
            Instruction::Right => &ways.1,
        }
    }
    index.to_string()
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

fn lcm(a: usize, b: usize) -> usize {
    // LCM = a*b / gcd
    return a * (b / gcd(a, b));
}

pub fn part2() -> String {
    let path = "input/day8.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let mut lines = input.lines();
    let instructions: Vec<Instruction> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => unreachable!(),
        })
        .collect::<Vec<Instruction>>();

    let mut keys: Vec<&str> = Vec::new();
    let mut hashmap = HashMap::new();
    lines
        .filter(|l| !l.is_empty())
        .map(|s| s.split(" = ").collect())
        .map(|lines: Vec<&str>| {
            if lines[0].ends_with("A") {
                keys.push(lines[0]);
            }
            (lines[0], lines[1].split(", ").collect::<Vec<&str>>())
        })
        .for_each(|(key, values)| {
            hashmap.insert(
                key,
                (values[0].replace("(", ""), values[1].replace(")", "")),
            );
        });

    keys.iter().map(|k| {
        let mut key = *k;
        let mut index = 0;
        let mut instr_iterator = instructions.iter();
        while !key.ends_with("Z") {
            let current_instr = match instr_iterator.next() {
                Some(instr) => instr,
                None => {
                    instr_iterator = instructions.iter();
                    instr_iterator.next().unwrap()
                }
            };
            index += 1;
            let ways = hashmap.get(key).unwrap();
            key = match current_instr {
                Instruction::Left => &ways.0,
                Instruction::Right => &ways.1,
            }
        }
        index
    }).fold(1, |acc, v| lcm(acc, v)).to_string()
}
