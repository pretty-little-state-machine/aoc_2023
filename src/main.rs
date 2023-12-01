mod day01;

use std::time::{Instant};
use colored::*;
use colored::Color::{Red, Green};


fn run_day(day: usize, func: fn(&str) -> (String, String), color: Color) {
    // Load the file before calling the function for accurate timing
    let start = Instant::now();
    let (p1, p2) = func("");
    let duration = start.elapsed();

    let title = match color {
        Red => String::from(format!("🎄Day {day} 🎄\n~~~~~~~~~~", )).bright_red(),
        Green => String::from(format!("🎄Day {day} 🎄\n~~~~~~~~~~", )).bright_green(),
        _ => String::from(format!("🎄Day {day} 🎄\n~~~~~~~~~~", )).white(),
    };
    println!("{title}\n{duration:?}\nPart 1: {p1}\nPart 2: {p2}\n");
}

fn main() {
    run_day(1, day01::run, Red);
    run_day(2, day01::run, Green);
}

