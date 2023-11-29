use std::path::Path;

use aoc_client::{last_unlocked_day, AocClient};
use clap::Parser;
use AoC_2023::{day_executer::execute_day, parser::CommandArgument};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command_argument = CommandArgument::parse();

    let day = match command_argument.day {
        Some(day) => day,
        None => last_unlocked_day(2023).unwrap(),
    };

    let part = match command_argument.part {
        Some(part) => part,
        None => 1,
    };

    let input_filename = format!("input/day{}.txt", day);

    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .day(day)?
        .year(2023)?
        .input_filename(&input_filename)
        .build()?;

    if !Path::new(&input_filename).exists() {
        client.save_input()?;
    }

    let result = execute_day(day, part);

    if command_argument.publish {
        client
            .submit_answer_and_show_outcome(&part.to_string(), result)
            .unwrap();
    } else {
        println!("{}", result);
    }

    Ok(())
}
