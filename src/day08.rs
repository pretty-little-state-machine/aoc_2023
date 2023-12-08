use crate::day08::Step::{Left, Right};
use crate::DayResult;
use fxhash::FxHashMap;
use num::Integer;
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
    //(None, (p1, p1_duration), (p2, p2_duration))
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

fn part_1(map: &Map, steps: &Steps) -> usize {
    let mut total_steps = 0;
    let step_length = steps.len();
    let mut current = "AAA".to_string();
    let mut step_cursor = 0;

    while let Some(entry) = map.get(&current) {
        let destination = match &steps[step_cursor] {
            Left => entry.left.clone(),
            Right => entry.right.clone(),
        };
        total_steps += 1;
        if destination == "ZZZ" {
            break;
        }
        current = destination;
        step_cursor += 1;
        step_cursor %= step_length;
    }
    total_steps
}

/// Returns the next entry in a map
fn step_through_map(source: String, map: &Map, step: &Step) -> String {
    let entry = map.get(&source).unwrap();
    match step {
        Left => entry.left.clone(),
        Right => entry.right.clone(),
    }
}

fn part_2(map: &Map, steps: &Steps) -> usize {
    let mut current_entries = map
        .values()
        .filter(|&e| e.source.ends_with('A'))
        .collect::<Vec<_>>();
    let mut destinations = Vec::with_capacity(current_entries.len());
    let mut steps_per_starting: FxHashMap<String, usize> = FxHashMap::default();
    let step_length = steps.len();

    let mut total_steps = 0;
    let mut step_cursor = 0;

    loop {
        total_steps += 1;
        for entry in &current_entries {
            destinations.push(step_through_map(
                entry.source.clone(),
                map,
                &steps[step_cursor],
            ));
        }
        step_cursor += 1;
        step_cursor %= step_length;
        current_entries.clear();
        for destination in &destinations {
            if destination.ends_with('Z') {
                steps_per_starting.insert(destination.clone(), total_steps);
            } else {
                current_entries.push(map.get(destination).unwrap());
            }
        }
        destinations.clear();
        if current_entries.is_empty() {
            break;
        }
    }
    let values = steps_per_starting.values().copied().collect::<Vec<usize>>();
    let mut lcm = 1;
    for v in values {
        lcm = lcm.lcm(&v);
    }
    lcm
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
