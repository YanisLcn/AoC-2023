pub fn execute_day(day: u32, part: u32) -> String {
    println!("Executing day {} part {}", day, part);
    match (day, part) {
        (1, 1) => crate::day1::part1(),
        (1, 2) => crate::day1::part2(),
        (2, 1) => crate::day2::part1(),
        (2, 2) => crate::day2::part2(),
        (3, 1) => crate::day3::part1(),
        (3, 2) => crate::day3::part2(),
        (4, 1) => crate::day4::part1(),
        (4, 2) => crate::day4::part2(),
        (5, 1) => crate::day5::part1(),
        (5, 2) => crate::day5::part2(),
        (6, 1) => crate::day6::part1(),
        (6, 2) => crate::day6::part2(),
        (7, 1) => crate::day7::part1(),
        (7, 2) => crate::day7::part2(),
        (8, 1) => crate::day8::part1(),
        (8, 2) => crate::day8::part2(),
        (9, 1) => crate::day9::part1(),
        (9, 2) => crate::day9::part2(),
        (10, 1) => crate::day10::part1(),
        (10, 2) => crate::day10::part2(),
        (11, 1) => crate::day11::part1(),
        (11, 2) => crate::day11::part2(),
        (12, 1) => crate::day12::part1(),
        (12, 2) => crate::day12::part2(),
        (13, 1) => crate::day13::part1(),
        (13, 2) => crate::day13::part2(),
        (14, 1) => crate::day14::part1(),
        (14, 2) => crate::day14::part2(),
        (15, 1) => crate::day15::part1(),
        (15, 2) => crate::day15::part2(),
        (16, 1) => crate::day16::part1(),
        (16, 2) => crate::day16::part2(),
        (17, 1) => crate::day17::part1(),
        (17, 2) => crate::day17::part2(),
        (18, 1) => crate::day18::part1(),
        (18, 2) => crate::day18::part2(),
        (19, 1) => crate::day19::part1(),
        (19, 2) => crate::day19::part2(),
        (20, 1) => crate::day20::part1(),
        (20, 2) => crate::day20::part2(),
        (21, 1) => crate::day21::part1(),
        (21, 2) => crate::day21::part2(),
        (22, 1) => crate::day22::part1(),
        (22, 2) => crate::day22::part2(),
        (23, 1) => crate::day23::part1(),
        (23, 2) => crate::day23::part2(),
        (24, 1) => crate::day24::part1(),
        (24, 2) => crate::day24::part2(),
        (25, 1) => crate::day25::part1(),
        (25, 2) => crate::day25::part2(),
        _ => panic!("Invalid day or part"),
    }
}
