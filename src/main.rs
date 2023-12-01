mod day01;

use colored::Color::{Green, Red};
use colored::*;
use std::fs;
use std::time::Instant;

fn run_day(day: usize, func: fn(&str) -> (String, String), color: Color) {
    // Load the file before calling the function for accurate timing
    let contents =
        fs::read_to_string(format!("./input/day_{:0>2}.txt", day)).expect("File not found.");

    let start = Instant::now();
    let (p1, p2) = func(&contents);
    let duration = start.elapsed();

    let title = match color {
        Red => format!("🎄Day {day} 🎄\n~~~~~~~~~~").bright_red(),
        Green => format!("🎄Day {day} 🎄\n~~~~~~~~~~").bright_green(),
        _ => format!("🎄Day {day} 🎄\n~~~~~~~~~~").white(),
    };
    println!("{title}\n{duration:?}\nPart 1: {p1}\nPart 2: {p2}\n");
}

fn main() {
    run_day(1, day01::run, Red);
}
