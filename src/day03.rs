use crate::DayResult;
use fxhash::{FxBuildHasher, FxHashMap, FxHashSet};
use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let (schematic, parts) = parse_schematic(input);
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let p1 = part_1(&schematic, &parts).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&schematic, &parts).to_string();
    let p2_duration = start.elapsed();
    (Some(parse_duration), (p1, p1_duration), (p2, p2_duration))
}

/// Part numbers may span across multiple grid entries.
type Schematic = FxHashMap<(isize, isize), Cell>;
type Parts = FxHashMap<usize, Part>;

#[derive(Debug, Default, Clone, Copy)]
struct Part {
    value: usize,
}

/// Cells may either have a blank ('.'), a symbol or a part number. Part numbers may span across
/// multiple cells. The same part ID is stored for every cell that has an identical part.
#[derive(Debug, Clone)]
enum Cell {
    Blank,
    Symbol { value: String },
    Part { part_id: usize },
}

fn get_part_value(decoder: &[usize]) -> usize {
    let mut value = 0;
    let mut offset = 1;

    for v in decoder.iter().rev() {
        value += v * offset;
        offset *= 10;
    }
    value
}

fn parse_schematic(input: &str) -> (Schematic, Parts) {
    let mut schematic = Schematic::with_capacity_and_hasher(5000, FxBuildHasher::default());
    let mut parts = Parts::with_capacity_and_hasher(500, FxBuildHasher::default());

    let mut part_id: usize = 0;
    let mut part_decoder: Vec<usize> = Vec::with_capacity(100);

    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            let x = x as isize;
            let y = y as isize;
            let mut flush_decoder = false;

            match cell {
                '0'..='9' => {
                    if part_decoder.is_empty() {
                        part_id += 1;
                    }
                    part_decoder.push(cell.to_string().parse::<usize>().unwrap());
                    schematic.insert((x, y), Cell::Part { part_id });
                }
                '.' => {
                    schematic.insert((x, y), Cell::Blank);
                    flush_decoder = true;
                }
                _ => {
                    schematic.insert(
                        (x, y),
                        Cell::Symbol {
                            value: cell.to_string(),
                        },
                    );
                    flush_decoder = true;
                }
            }
            if flush_decoder && !part_decoder.is_empty() {
                parts.insert(
                    part_id,
                    Part {
                        value: get_part_value(&part_decoder),
                    },
                );
                part_decoder.clear();
            }
        }
    }
    (schematic, parts)
}

#[inline(always)]
fn cell_has_adjacent_symbol(schematic: &Schematic, x: isize, y: isize) -> bool {
    let mut found = false;
    let cells_to_check = [
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ];
    for point in cells_to_check {
        if let Some(Cell::Symbol { .. }) = schematic.get(&point) {
            found = true
        }
    }
    found
}

fn find_gear_ratio(schematic: &Schematic, parts: &Parts, x: isize, y: isize) -> usize {
    let mut found_part_ids = FxHashSet::default();
    let cells_to_check = [
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ];
    for point in cells_to_check {
        if let Some(Cell::Part { part_id }) = schematic.get(&point) {
            found_part_ids.insert(part_id);
        }
    }

    if found_part_ids.len() == 2 {
        let mut result = 1;
        for c in found_part_ids {
            result *= parts.get(c).unwrap().value
        }
        result
    } else {
        0
    }
}

/// Find the sum of all parts adjacent to a symbol in the schematic, including diagonals.
fn part_1(schematic: &Schematic, parts: &Parts) -> usize {
    let mut valid_part_ids: FxHashSet<usize> = FxHashSet::default();

    for ((x, y), cell) in schematic {
        if let Cell::Part { part_id } = cell {
            if cell_has_adjacent_symbol(schematic, *x, *y) {
                valid_part_ids.insert(*part_id);
            }
        }
    }

    let mut total = 0;
    for id in valid_part_ids {
        total += parts.get(&id).unwrap().value
    }
    total
}

/// Find the sum of all gear ratios. Gears are '*' symbols adjacent to two part values. The gear
/// ratio is the product of both adjacent part values.
fn part_2(schematic: &Schematic, parts: &Parts) -> usize {
    let mut ratio_total = 0;
    let gear_cells: Vec<_> = schematic
        .iter()
        .filter(|&((_x, _y), cell)| {
            if let Cell::Symbol { value } = cell {
                value == "*"
            } else {
                false
            }
        })
        .collect();

    for ((x, y), _) in gear_cells {
        ratio_total += find_gear_ratio(schematic, parts, *x, *y);
    }
    ratio_total
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part_1() {
        let (schematic, parts) = parse_schematic(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(part_1(&schematic, &parts), 4361);
    }

    #[test]
    fn test_part_2() {
        let (schematic, parts) = parse_schematic(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(part_2(&schematic, &parts), 467835);
    }

    #[test]
    fn test_get_part_value() {
        assert_eq!(get_part_value(&vec![1, 2, 3]), 123);
        assert_eq!(get_part_value(&vec![4, 5, 6, 7, 8]), 45678);
    }
}
