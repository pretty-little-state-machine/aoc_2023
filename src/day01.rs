use crate::DayResult;
use rayon::prelude::*;
use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let p1 = part_1(input).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(input).to_string();
    let p2_duration = start.elapsed();
    (None, (p1, p1_duration), (p2, p2_duration))
}

#[inline(always)]
fn part_1(input: &str) -> usize {
    input.par_lines().map(get_line_value).sum()
}

#[inline(always)]
fn part_2(input: &str) -> usize {
    input
        .par_lines()
        .map(|l| get_line_value(replace_words(l).as_str()))
        .sum()
}

/// Finds the first and last digits within a line and creates a two-digit number from them.
#[inline(always)]
fn get_line_value(input: &str) -> usize {
    let mut first = 0;
    let mut last = 0;
    for mut c in input.bytes() {
        c -= 48;
        if c <= 9 {
            last = c;
            if first == 0 {
                first = c;
            }
        }
    }
    first as usize * 10 + last as usize
}

/// We replace words with a number, but we MUST keep the first and last letters in case there are
/// words like `oneeight` or `eightwo`. Sneaky for day 1!
#[inline(always)]
fn replace_words(input: &str) -> String {
    input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part_1() {
        let result = part_1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
",
        );
        assert_eq!(result, 142);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
",
        );
        assert_eq!(result, 281);
    }

    #[test]
    fn test_overlapping_words() {
        assert_eq!(part_2("oneight"), 18);
        assert_eq!(part_2("nineight"), 98);
        assert_eq!(part_2("eightwo"), 82);
    }
}
