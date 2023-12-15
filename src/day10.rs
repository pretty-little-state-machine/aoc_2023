use crate::day10::PipeKind::{
    Ground, Horizontal, NorthEast, NorthWest, SouthEast, SouthWest, Start, Vertical,
};
use crate::DayResult;
use colored::Color::{BrightBlue, Red, White};
use colored::Colorize;
use fxhash::FxHashMap;
use itertools::Itertools;
use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let mut network = parse_network(input);
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let p1 = part_1(&mut network).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&mut network).to_string();
    let p2_duration = start.elapsed();
    (Some(parse_duration), (p1, p1_duration), (p2, p2_duration))
}

type Point = (isize, isize);
type PipeNetwork = FxHashMap<Point, Pipe>;

#[derive(Debug, Clone)]
struct Pipe {
    kind: PipeKind,
    visited: bool,
    internal: bool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
    let pipes = network.iter().sorted_by_key(|(k, _v)| (k.0, k.1));
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
        } else if pipe.internal {
            print!("{}", "▒".to_string().color(Red));
        } else {
            print!("{}", c.to_string().color(White));
        }
    }
    println!();
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
                    internal: false,
                },
            );
        }
    }
    network
}

fn travel_pipe(network: &mut PipeNetwork, point: &Point) -> Option<Point> {
    let current_kind = network.get(point).unwrap().kind;
    let mut target = None;
    // Eastwards
    if let Some(pipe) = network.get_mut(&(point.0, point.1 + 1)) {
        if !pipe.visited {
            match (current_kind, &pipe.kind) {
                (Horizontal, Horizontal) | (Horizontal, NorthWest) | (Horizontal, SouthWest) => {
                    pipe.visited = true;
                    target = Some((point.0, point.1 + 1))
                }
                (NorthEast, Horizontal) | (NorthEast, SouthWest) | (NorthEast, NorthWest) => {
                    pipe.visited = true;
                    target = Some((point.0, point.1 + 1))
                }
                (SouthEast, Horizontal) | (SouthEast, SouthWest) | (SouthEast, NorthWest) => {
                    pipe.visited = true;
                    target = Some((point.0, point.1 + 1))
                }
                (Start, Horizontal) | (Start, SouthWest) | (Start, NorthEast) => {
                    pipe.visited = true;
                    target = Some((point.0, point.1 + 1))
                }
                (_, _) => (),
            }
        }
    }
    // Westwards
    if let Some(pipe) = network.get_mut(&(point.0, point.1 - 1)) {
        if !pipe.visited {
            match (current_kind, &pipe.kind) {
                (Horizontal, Horizontal) | (Horizontal, NorthEast) | (Horizontal, SouthEast) => {
                    pipe.visited = true;
                    target = Some((point.0, point.1 - 1))
                }
                (NorthWest, Horizontal) | (NorthWest, NorthEast) | (NorthWest, SouthEast) => {
                    pipe.visited = true;
                    target = Some((point.0, point.1 - 1))
                }
                (SouthWest, Horizontal) | (SouthWest, NorthEast) | (SouthWest, SouthEast) => {
                    pipe.visited = true;
                    target = Some((point.0, point.1 - 1))
                }
                (Start, Horizontal) | (Start, NorthEast) | (Start, SouthEast) => {
                    pipe.visited = true;
                    target = Some((point.0, point.1 - 1))
                }
                (_, _) => (),
            }
        }
    }
    // Northwards
    if let Some(pipe) = network.get_mut(&(point.0 - 1, point.1)) {
        if !pipe.visited {
            match (current_kind, &pipe.kind) {
                (Vertical, Vertical) | (Vertical, SouthWest) | (Vertical, SouthEast) => {
                    pipe.visited = true;
                    target = Some((point.0 - 1, point.1))
                }
                (NorthEast, Vertical) | (NorthEast, SouthWest) | (NorthEast, SouthEast) => {
                    pipe.visited = true;
                    target = Some((point.0 - 1, point.1))
                }
                (NorthWest, Vertical) | (NorthWest, SouthWest) | (NorthWest, SouthEast) => {
                    pipe.visited = true;
                    target = Some((point.0 - 1, point.1))
                }
                (Start, Vertical) | (Start, SouthWest) | (Start, SouthEast) => {
                    pipe.visited = true;
                    target = Some((point.0 - 1, point.1))
                }
                (_, _) => (),
            }
        }
    }
    // Southwards
    if let Some(pipe) = network.get_mut(&(point.0 + 1, point.1)) {
        if !pipe.visited {
            match (current_kind, &pipe.kind) {
                (Vertical, Vertical) | (Vertical, NorthEast) | (Vertical, NorthWest) => {
                    pipe.visited = true;
                    target = Some((point.0 + 1, point.1))
                }
                (SouthWest, Vertical) | (SouthWest, NorthEast) | (SouthWest, NorthWest) => {
                    pipe.visited = true;
                    target = Some((point.0 + 1, point.1))
                }
                (SouthEast, Vertical) | (SouthEast, NorthEast) | (SouthEast, NorthWest) => {
                    pipe.visited = true;
                    target = Some((point.0 + 1, point.1))
                }
                (Start, Vertical) | (Start, NorthEast) | (Start, NorthWest) => {
                    pipe.visited = true;
                    target = Some((point.0 + 1, point.1))
                }
                (_, _) => (),
            }
        }
    }
    target
}

fn part_1(network: &mut PipeNetwork) -> isize {
    let (start, pipe) = network
        .iter_mut()
        .find(|(_, pipe)| pipe.kind == Start)
        .expect("Couldn't find starting location.");
    pipe.visited = true;
    let start = *start;
    let mut current_hop = start;
    let mut total_steps = 2;
    while let Some(x) = travel_pipe(network, &current_hop) {
        total_steps += 1;
        current_hop = x;
    }
    // draw_network(network);
    total_steps / 2
}

/// Part 2 requires the traversal in Part 1 to be complete.
fn part_2(network: &mut PipeNetwork) -> usize {
    let scan_network = network.clone();
    let pipes = network.iter_mut().sorted_by_key(|(k, _v)| (k.0, k.1));
    let mut cur_row: isize = 0;

    let mut internal_pipes = 0;
    let mut outside_edge = true;

    for ((row, col), pipe) in pipes {
        if *row > cur_row {
            cur_row = *row;
            outside_edge = true;
        }
        let mut col_offset: isize = 1;
        let mut crossings = 0;
        let mut prev_pipe_visited = pipe.visited;
        let mut prev_pipe_kind = pipe.kind;
        while let Some(next_pipe) = scan_network.get(&(cur_row, col + col_offset)) {
            // If we hit an `╔` or `╚` pipe then it's possible to travel down the network for a bit
            // before leaving the intersection again since we are scanning eastwards. Only by
            // hitting a `╗` or `╝` pipe can we consider the horizontal span finished.
            match next_pipe.kind {
                NorthWest | SouthWest => {
                    if next_pipe.visited {
                        if prev_pipe_visited && prev_pipe_kind == NorthEast && next_pipe.kind == NorthWest {
                            crossings += 1;

                        } else if prev_pipe_visited && prev_pipe_kind == SouthEast && next_pipe.kind == SouthWest {
                            crossings += 1;
                        }
                    }
                }
                NorthEast | SouthEast => {
                    if next_pipe.visited {
                        crossings += 1;
                    }
                },
                Vertical=> {
                    if next_pipe.visited {
                        crossings += 1;
                    }
                },
                _ => (),
            }
            prev_pipe_visited = next_pipe.visited;
            prev_pipe_kind = next_pipe.kind;
            col_offset += 1;
        }
        if pipe.visited {
            outside_edge = false;
        }
        if crossings % 2 != 0 && !pipe.visited && !outside_edge {
            pipe.internal = true;
            internal_pipes += 1;
        }
    }
    draw_network(&network);
    internal_pipes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_sample_1() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        let mut network = parse_network(input);
        assert_eq!(part_1(&mut network), 4);
    }

    #[test]
    fn test_part_1_sample_2() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        let mut network = parse_network(input);
        assert_eq!(part_1(&mut network), 8);
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let mut network = parse_network(input);
        part_1(&mut network);
        assert_eq!(part_2(&mut network), 10);
    }
}
