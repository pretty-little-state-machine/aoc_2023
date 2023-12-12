use crate::DayResult;
use itertools::Itertools;
use num::abs;
use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let p1 = part_1(input).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(input, 1_000_000 - 1).to_string();
    let p2_duration = start.elapsed();
    (None, (p1, p1_duration), (p2, p2_duration))
}

type Point = (isize, isize);
type Image = Vec<Point>;

fn parse_map(input: &str) -> Image {
    let mut image = Image::default();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                '#' => {
                    let p: Point = (row as isize + 1, col as isize + 1);
                    image.push(p);
                }
                _ => unreachable!("Unknown image character!"),
            }
        }
    }
    image
}

fn run_expansion(image: &mut Image, steps: isize) {
    let mut rows_to_expand: Vec<isize> = Vec::default();
    let mut cols_to_expand: Vec<isize> = Vec::default();

    let highest_row = image.iter().max_by_key(|&p| p.0).unwrap().0;
    let highest_col = image.iter().max_by_key(|&p| p.1).unwrap().1;

    let mut rows_offset = 0;
    for r in 1..=highest_row {
        if !image.iter().any(|&p| p.0 == r) {
            rows_to_expand.push(r + rows_offset);
            rows_offset += steps;
        }
    }

    let mut cols_offset = 0;
    for c in 1..=highest_col {
        if !image.iter().any(|&p| p.1 == c) {
            cols_to_expand.push(c + cols_offset);
            cols_offset += steps;
        }
    }
    // Expand space
    for point in &mut *image {
        for row in &rows_to_expand {
            if *row < point.0 {
                point.0 += steps;
            }
        }
        for col in &cols_to_expand {
            if *col < point.1 {
                point.1 += steps;
            }
        }
    }
}

fn calc_distances(image: &Image) -> isize {
    image
        .iter()
        .combinations(2)
        .map(|perm| {
            let a = perm.first().unwrap();
            let b = perm.last().unwrap();
            abs(a.0 - b.0) + abs(a.1 - b.1)
        })
        .sum()
}

fn part_1(input: &str) -> isize {
    let mut image = parse_map(input);
    run_expansion(&mut image, 1);
    calc_distances(&image)
}

fn part_2(input: &str, steps: isize) -> isize {
    let mut image = parse_map(input);
    run_expansion(&mut image, steps);
    calc_distances(&image)
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
        assert_eq!(part_1(input), 374);
    }

    #[test]
    fn test_part_2() {
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
        assert_eq!(part_2(input, 99), 8410);
    }
}
