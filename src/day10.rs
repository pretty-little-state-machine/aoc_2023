use crate::day10::PipeKind::{
    Ground, Horizontal, NorthEast, NorthWest, SouthEast, SouthWest, Start, Vertical,
};
use crate::DayResult;
use fxhash::FxHashMap;
use itertools::Itertools;
use std::time::Instant;
use colored::Color::{BrightBlue, Red, White};
use colored::Colorize;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let p1 = part_1(&input).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&input).to_string();
    let p2_duration = start.elapsed();
    // (Some(parse_duration), (p1, p1_duration), (p2, p2_duration))
    (None, (p1, p1_duration), (p2, p2_duration))
}

type Point = (isize, isize);
type PipeNetwork = FxHashMap<Point, Pipe>;

#[derive(Debug, Clone)]
struct Pipe {
    kind: PipeKind,
    visited: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum PipeKind {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl PipeKind {
    fn new(c: char) -> Self {
        match c {
            '|' => Vertical,
            '-' => Horizontal,
            'L' => NorthEast,
            'J' => NorthWest,
            '7' => SouthWest,
            'F' => SouthEast,
            '.' => Ground,
            'S' => Start,
            _ => unreachable!("Unknown tile"),
        }
    }
}

fn draw_network(network: &PipeNetwork) {
    let pipes = network.iter().sorted_by_key(|(k, v)| (k.0, k.1));
    let mut cur_row = 0;
    for ((row, _), pipe) in pipes {
        if cur_row != *row {
            println!();
            cur_row = *row;
        }
        let c = match pipe.kind {
            Vertical => '║',
            Horizontal => '═',
            NorthEast => '╚',
            NorthWest => '╝',
            SouthWest => '╗',
            SouthEast => '╔',
            Ground => ' ',
            Start => '◎',
        };
        if pipe.visited {
            print!("{}", c.to_string().color(BrightBlue));
        } else {
            print!("{}", c.to_string().color(White));
        }
    }
}

fn parse_network(input: &str) -> PipeNetwork {
    let mut network = PipeNetwork::default();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let p: Point = (row as isize, col as isize);
            network.insert(
                p,
                Pipe {
                    kind: PipeKind::new(c),
                    visited: false,
                },
            );
        }
    }
    network
}

fn travel_pipe(network: &mut PipeNetwork, point: &Point) -> Option<Point> {
    let current_kind = network.get(&point).unwrap().kind.clone();
    // East
    if let Some(pipe) = network.get_mut(&(point.0, point.1 + 1)) {
        if pipe.kind == Horizontal || pipe.kind == NorthWest || pipe.kind == SouthWest {
            pipe.visited = true;
            return Some((point.0, point.1 + 1));
        }
    }
    // West
    if let Some(pipe) = network.get_mut(&(point.0, point.1 - 1)) {
        if pipe.kind == Horizontal || pipe.kind == NorthEast || pipe.kind == SouthEast {
            pipe.visited = true;
            return Some((point.0, point.1 + 1));
        }
    }
    // North
    if let Some(pipe) = network.get_mut(&(point.0 - 1, point.1)) {
        if pipe.kind == Vertical || pipe.kind == SouthEast || pipe.kind == SouthWest {
            pipe.visited = true;
            return Some((point.0 - 1, point.1));
        }
    }
    // South
    if let Some(pipe) = network.get_mut(&(point.0 + 1, point.1)) {
        if pipe.kind == Vertical || pipe.kind == NorthEast || pipe.kind == NorthWest {
            pipe.visited = true;
            return Some((point.0 + 1, point.1));
        }
    }
    None
}

fn part_1(input: &str) -> usize {
    let mut network = parse_network(input);
    let (start, pipe) = network
        .iter_mut()
        .find(|(_, pipe)| pipe.kind == Start)
        .expect("Couldn't find starting location.");
    pipe.visited = true;
    let mut current_hop = start.clone();
    while let Some(x) = travel_pipe(&mut network, &current_hop) {
        current_hop = x;
        println!("{current_hop:?}");
    }
    draw_network(&network);
    0
}

fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        let network = parse_network(input);
        part_1(input);
    }

    #[test]
    fn test_part_2() {}
}
