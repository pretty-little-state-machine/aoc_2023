mod day01;
mod day02;

use colored::Color::{Green, Red};
use colored::*;
use std::fs;
use std::time::Duration;

type DayResult = (Option<Duration>, (String, Duration), (String, Duration));

fn run_day(day: usize, func: fn(&str) -> DayResult, color: Color) {
    // Load the file before calling the function for accurate timing
    let contents =
        fs::read_to_string(format!("./input/day_{:0>2}.txt", day)).expect("File not found.");

    let (parse_duration, (p1, p1_duration), (p2, p2_duration)) = func(&contents);
    let mut total_duration = p1_duration + p2_duration;
    if let Some(p) = parse_duration {
        total_duration += p;
    }

    let title = match color {
        Red => format!("🎄Day {day} ({total_duration:?}) 🎄\n~~~~~~~~~~~~~~~~~~~").bright_red(),
        Green => format!("🎄Day {day} ({total_duration:?}) 🎄\n~~~~~~~~~~~~~~~~~~~").bright_green(),
        _ => format!("🎄Day {day} ({total_duration:?}) 🎄\n~~~~~~~~~~~~~~~~~~~").white(),
    };
    println!("{title}");
    if let Some(p) = parse_duration {
        println!("Parse : ({p:?})");
    }
    print!("{}", "Part 1: ".white());
    print!("{}", p1.as_str().bold().white());
    println!(" ({p1_duration:?})");
    print!("{}", "Part 2: ".white());
    print!("{}", p2.as_str().bold().white());
    println!(" ({p2_duration:?})\n");
}

fn main() {
    run_day(1, day01::run, Red);
    run_day(2, day02::run, Green);
}
