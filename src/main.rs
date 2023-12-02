mod day01;

use colored::Color::{Green, Red};
use colored::*;
use std::fs;
use std::time::Duration;

type DayResult = ((String, Duration), (String, Duration));

fn run_day(day: usize, func: fn(&str) -> DayResult, color: Color) {
    // Load the file before calling the function for accurate timing
    let contents =
        fs::read_to_string(format!("./input/day_{:0>2}.txt", day)).expect("File not found.");

    let ((p1, p1_duration), (p2, p2_duration)) = func(&contents);
    let total_duration = p1_duration + p2_duration;

    let title = match color {
        Red => format!("🎄Day {day} ({total_duration:?}) 🎄\n~~~~~~~~~~~~~~~~~~~").bright_red(),
        Green => format!("🎄Day {day} ({total_duration:?}) 🎄\n~~~~~~~~~~~~~~~~~~~").bright_green(),
        _ => format!("🎄Day {day} ({total_duration:?}) 🎄\n~~~~~~~~~~~~~~~~~~~").white(),
    };
    println!("{title}\nPart 1: {p1} ({p1_duration:?})\nPart 2: {p2} ({p2_duration:?})\n");
}

fn main() {
    run_day(1, day01::run, Red);
}
