use crate::DayResult;
use fxhash::{FxHashMap, FxHashSet};
use std::time::Instant;

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
type Image = FxHashSet<Point>;

fn parse_map(input: &str) -> Image {
    let mut image = Image::default();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                '#' => {
                    let p: Point = (row as isize + 1, col as isize + 1);
                    image.insert(p);
                }
                _ => unreachable!("Unknown image character!"),
            }
        }
    }
    image
}

fn run_expansion(image: &Image, steps: usize) {
    let mut rows_to_expand: FxHashSet<isize> = FxHashSet::default();
    let mut cols_to_expand: FxHashSet<isize> = FxHashSet::default();

    let highest_row = image.iter().max_by_key(|&p| p.0).unwrap().0;
    let highest_col = image.iter().max_by_key(|&p| p.1).unwrap().1;

    for r in 1..=highest_row {
        if !image.iter().any(|&p| p.0 == r) {
            rows_to_expand.insert(r);
        }
    }

    for c in 1..=highest_col {
        if !image.iter().any(|&p| p.1 == c) {
            cols_to_expand.insert(c);
        }
    }

    println!("Row: {highest_row:?}, Col: {highest_col:?}");
    println!("Rows to expand: {rows_to_expand:?}");
    println!("Cols to expand: {cols_to_expand:?}");
}

fn part_1(input: &str) -> usize {
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
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        let image = parse_map(input);
        println!("{image:?}");
        run_expansion(&image, 1);
        assert_eq!(part_1(input), 374);
    }

    #[test]
    fn test_part_2() {}
}
