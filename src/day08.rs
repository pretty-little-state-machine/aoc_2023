use crate::day08::Step::{Left, Right};
use crate::DayResult;
use fxhash::FxHashMap;
use num::Integer;
use rayon::prelude::*;
use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let (map, steps) = parse(input);
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let p1 = part_1(&map, &steps).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&map, &steps).to_string();
    let p2_duration = start.elapsed();
    (Some(parse_duration), (p1, p1_duration), (p2, p2_duration))
}

type Map = FxHashMap<String, Entry>;
type Steps = Vec<Step>;

#[derive(Debug)]
enum Step {
    Left,
    Right,
}

impl Step {
    fn from_char(c: char) -> Self {
        match c {
            'R' => Right,
            'L' => Left,
            _ => unreachable!("Unknown movement"),
        }
    }
}

#[derive(Debug)]
struct Entry {
    source: String,
    left: String,
    right: String,
}

fn parse(input: &str) -> (Map, Steps) {
    let mut map = FxHashMap::default();
    let mut lines = input.lines();
    let steps = lines
        .next()
        .unwrap()
        .chars()
        .map(Step::from_char)
        .collect::<Vec<Step>>();
    lines.next(); // Blank line;
    for line in lines {
        let mut fields = line.split_ascii_whitespace();
        let source = fields.next().unwrap().to_string();
        fields.next(); // '='
        let left = fields.next().unwrap();
        let right = fields.next().unwrap();

        map.insert(
            source.clone(),
            Entry {
                source: source.clone(),
                left: left.replace(['(', ','], "").to_string(),
                right: right.replace(')', "").to_string(),
            },
        );
    }
    (map, steps)
}

/// Returns the steps required to reach the destination
fn find_path_steps(source: String, map: &Map, steps: &Steps, part_2: bool) -> usize {
    let step_length = steps.len();
    let mut current = source;
    let mut step_cursor = 0;
    let mut total_steps = 0;
    while let Some(entry) = map.get(&current) {
        let destination = match &steps[step_cursor] {
            Left => entry.left.clone(),
            Right => entry.right.clone(),
        };
        total_steps += 1;
        if (!part_2 && destination.contains("ZZZ")) || (part_2 && destination.ends_with('Z')) {
            break;
        }
        current = destination;
        step_cursor += 1;
        step_cursor %= step_length;
    }
    total_steps
}

fn part_1(map: &Map, steps: &Steps) -> usize {
    find_path_steps("AAA".to_string(), map, steps, false)
}

fn part_2(map: &Map, steps: &Steps) -> usize {
    let results = map
        .values()
        .filter(|&e| e.source.ends_with('A'))
        .collect::<Vec<_>>()
        .par_iter()
        .map(|e| find_path_steps(e.source.clone(), map, steps, true))
        .collect::<Vec<usize>>();

    results.iter().fold(1, |acc, r| acc.lcm(r))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_ex1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
        let (map, steps) = parse(input);
        let result = part_1(&map, &steps);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_1_ex2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
        let (map, steps) = parse(input);
        let result = part_1(&map, &steps);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part_2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
        let (map, steps) = parse(input);
        let result = part_2(&map, &steps);
        assert_eq!(result, 6);
    }
}
