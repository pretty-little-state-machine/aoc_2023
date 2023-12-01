use rayon::prelude::*;

pub fn run(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(input: &str) -> usize {
    input.par_lines().map(get_line_value).sum()
}

fn part_2(input: &str) -> usize {
    input.par_lines().map(|l| get_line_value(replace_words(l).as_str())).sum()
}

/// Finds the first and last digits within a line and creates a two-digit number from them.
fn get_line_value(input: &str) -> usize {
    let mut values = Vec::new();
    for c in input.chars() {
        if let Ok(usize) = c.to_string().parse::<usize>() {
            values.push(usize)
        }
    }
    values.first().unwrap() * 10 + values.last().unwrap()
}

/// We replace words with a number, but we MUST keep the first and last letters in case there are
/// words like `oneeight` or `eightwo`. Sneaky for day 1!
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
