use std::path::Path;

use aoc_2023::{day_executer::execute_day, parser::CommandArgument};
use aoc_client::{last_unlocked_day, AocClient};
use clap::Parser;

fn run_day(
    year: i32,
    day: u32,
    part: u32,
    publish: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("input")?;

    let input_filename = format!("input/day{}.txt", day);

    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(year)?
        .day(day)?
        .input_filename(&input_filename)
        .build()?;

    if !Path::new(&input_filename).exists() {
        client.save_input()?;
    }

    let result = execute_day(day, part);

    if publish {
        client
            .submit_answer_and_show_outcome(&part.to_string(), result)
            .unwrap();
    } else {
        println!("{}", result);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let command_argument = CommandArgument::parse();
    let year = 2023;

    if command_argument.all {
        for day in 1..=last_unlocked_day(year).expect("AoC 2023 is not unlocked yet") {
            for part in 1..=2 {
                println!("Executing day {} part {}", day, part);
                run_day(year, day, part, command_argument.publish)?;
            }
        }

        return Ok(());
    }

    let day = match command_argument.day {
        Some(day) => day,
        None => last_unlocked_day(year).expect("AoC 2023 is not unlocked yet"),
    };

    let part = command_argument.part.unwrap_or(1);

    println!("Executing day {} part {}", day, part);
    run_day(year, day, part, command_argument.publish)?;

    Ok(())
}
