use crate::DayResult;
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

fn build_differences(nums: &[isize]) -> Vec<isize> {
    nums.windows(2).map(|w| w[1] - w[0]).collect()
}

fn parse_line(input: &str) -> Vec<isize> {
    input
        .split_ascii_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .collect()
}

fn find_next_value(nums: Vec<isize>) -> isize {
    let mut stack: Vec<Vec<isize>> = Vec::default();
    stack.push(nums);
    loop {
        let p = build_differences(stack.last().unwrap());
        if p.iter().all(|&x| x == 0) {
            break;
        }
        stack.push(p);
    }
    stack.reverse();
    let mut last_value = *stack.first().unwrap().last().unwrap();
    let mut s_iter = stack.iter();
    s_iter.next();
    for s in s_iter {
        last_value += *s.last().unwrap();
    }
    last_value
}

fn part_1(input: &str) -> isize {
    input.lines().map(|l| find_next_value(parse_line(l))).sum()
}

fn part_2(input: &str) -> isize {
    input
        .lines()
        .map(|l| {
            let mut values = parse_line(l);
            values.reverse();
            find_next_value(values)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(part_1(input), 114);
    }

    #[test]
    fn test_part_2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(part_2(input), 2);
    }
}
