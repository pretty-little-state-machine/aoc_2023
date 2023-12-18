use crate::day14::Direction::{East, North, South, West};
use crate::day14::Rock::Round;
use crate::DayResult;
use colored::Colorize;
use fxhash::FxHashMap;
use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let mut platform = parse_input(input);
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let p1 = part_1(&mut platform.clone()).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&mut platform).to_string();
    let p2_duration = start.elapsed();
    (Some(parse_duration), (p1, p1_duration), (p2, p2_duration))
}

type Platform = FxHashMap<Point, Rock>;
type Point = (isize, isize);

#[derive(Debug, Clone, PartialEq, Hash)]
enum Rock {
    Cube,
    Round,
}

impl Rock {
    fn new(input: char) -> Option<Self> {
        match input {
            'O' => Some(Rock::Round),
            '#' => Some(Rock::Cube),
            _ => None,
        }
    }
}

fn parse_input(input: &str) -> Platform {
    let mut platform = Platform::default();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if let Some(rock) = Rock::new(c) {
                platform.insert((row as isize, col as isize), rock);
            }
        }
    }
    platform
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn draw_platform(platform: &Platform, max_point: Point) {
    for row in 0..=max_point.0 {
        for col in 0..=max_point.1 {
            if let Some(rock) = platform.get(&(row, col)) {
                let output = match rock {
                    Rock::Cube => "#".bright_yellow(),
                    Rock::Round => "O".bright_purple(),
                };
                print!("{}", output);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

/// Simulates shifting the platform a certain direction. Returns the number of stones that moved.
#[inline(always)]
fn tick_platform(platform: &mut Platform, direction: &Direction, max_point: Point) -> usize {
    let mut moved_stones = 0;

    let rock_offset: Point = match direction {
        North => (-1, 0),
        South => (1, 0),
        East => (0, 1),
        West => (0, -1),
    };
    // I thought it would be faster to flip around row/column order based on direction but that is
    // actually ~30% slower when running the full day. There's optimizations here somewhere though.
    for row in 0..=max_point.0 {
        for col in 0..=max_point.1 {
            let target = (row + rock_offset.0, col + rock_offset.1);
            // Rocks can not slide off of the platform.
            if target.0 < 0 || target.1 < 0 || target.0 > max_point.0 || target.1 > max_point.1 {
                continue;
            }
            if platform.contains_key(&(row, col))
                && !platform.contains_key(&target)
                && *platform.get(&(row, col)).unwrap() == Round
            {
                let rock = platform.remove(&(row, col)).unwrap();
                platform.insert(target, rock);
                moved_stones += 1;
            }
        }
    }
    moved_stones
}

#[inline(always)]
fn get_max_point(platform: &Platform) -> Point {
    (
        platform.iter().max_by_key(|((r, _c), _v)| r).unwrap().0 .0,
        platform.iter().max_by_key(|((_r, c), _v)| c).unwrap().0 .1,
    )
}

fn calc_load(platform: &Platform, max_point: Point) -> isize {
    let mut load = 0;
    for row in 0..=max_point.0 {
        for col in 0..=max_point.1 {
            if let Some(rock) = platform.get(&(row, col)) {
                load += match rock {
                    Rock::Cube => 0,
                    Rock::Round => max_point.1 - row + 1,
                };
            }
        }
    }
    load
}

fn part_1(platform: &mut Platform) -> isize {
    let max_point = get_max_point(platform);
    loop {
        let moved_stones = tick_platform(platform, &North, max_point);
        /* Visualization
        draw_platform(platform, max_point);
        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);
        print!("\x1B[2J\x1B[1;1H");
        */
        if moved_stones == 0 {
            break;
        }
    }
    calc_load(platform, max_point)
}

#[inline(always)]
fn roll_rocks_until_stopped(platform: &mut Platform, direction: &Direction, max_point: Point) {
    loop {
        if tick_platform(platform, direction, max_point) == 0 {
            break;
        }
    }
}

fn part_2(platform: &mut Platform) -> isize {
    let max_point = get_max_point(platform);
    let mut seen_positions: Vec<Platform> = Vec::with_capacity(20_000);

    let mut seen_first_at = 0;

    loop {
        roll_rocks_until_stopped(platform, &North, max_point);
        roll_rocks_until_stopped(platform, &West, max_point);
        roll_rocks_until_stopped(platform, &South, max_point);
        roll_rocks_until_stopped(platform, &East, max_point);
        let snapshot = platform.clone();
        if let Some(index) = seen_positions.iter().position(|p| p == &snapshot) {
            if seen_first_at == 0 {
                seen_first_at = index;
                let tmp = seen_positions.last().unwrap().clone();
                seen_positions.clear();
                seen_positions.push(tmp);
            } else {
                break;
            }
        }
        seen_positions.push(snapshot);
    }
    let num_remaining_steps = (1_000_000_000 - seen_first_at) % (seen_positions.len());
    for _ in 0..num_remaining_steps {
        roll_rocks_until_stopped(platform, &North, max_point);
        roll_rocks_until_stopped(platform, &West, max_point);
        roll_rocks_until_stopped(platform, &South, max_point);
        roll_rocks_until_stopped(platform, &East, max_point);
    }
    // draw_platform(platform, max_point);
    calc_load(platform, max_point)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let mut platform = parse_input(&input);
        assert_eq!(part_1(&mut platform), 136);
    }

    #[test]
    fn test_part_2() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let mut platform = parse_input(&input);
        assert_eq!(part_2(&mut platform), 64);
    }
}
