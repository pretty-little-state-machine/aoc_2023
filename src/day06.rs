use crate::DayResult;
use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let races = parse(input);
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let p1 = part_1(&races).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&races).to_string();
    let p2_duration = start.elapsed();
    (Some(parse_duration), (p1, p1_duration), (p2, p2_duration))
}

type Races = Vec<Race>;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    /// Distance = Time_Held * (Total_Time - Time_Held) ==> d=th-h^2 which is parabolic
    /// 0 = -h^2 + th - d  or more commonly: `0 = -ax^2 + bx - c`
    fn num_winning_moves(&self) -> usize {
        let a = -1.0;
        let b = self.time as f64;
        let c = -(self.distance as f64);

        let desc = (b.powi(2) - (4.0 * a * c)).sqrt();
        let root_a = (-b + desc) / (2.0 * a);
        let root_b = (-b - desc) / (2.0 * a);

        // Perfect squares mess with this! Remove edge overlaps where we match the winning distance
        let mut offset = 0.0;
        if root_a == root_a.ceil() {
            offset -= 1.0;
        }
        if root_b == root_b.floor() {
            offset -= 1.0;
        }
        (root_b.floor() - root_a.ceil() + 1.0 + offset) as usize
    }
}

fn parse(input: &str) -> Races {
    let mut rows = input.lines();
    let times = rows.next().unwrap().split_ascii_whitespace();
    let distances = rows.next().unwrap().split_ascii_whitespace();
    let mut pairs = times.zip(distances);

    pairs.next(); // Skip headers
    pairs
        .map(|(time, distance)| Race {
            time: time.parse::<usize>().unwrap(),
            distance: distance.parse::<usize>().unwrap(),
        })
        .collect()
}

fn part_1(races: &Races) -> usize {
    races.iter().map(|r| r.num_winning_moves()).product()
}

fn get_p2_input(races: &Races) -> Race {
    Race {
        time: races
            .iter()
            .map(|r| r.time.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<usize>()
            .unwrap(),
        distance: races
            .iter()
            .map(|r| r.distance.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<usize>()
            .unwrap(),
    }
}

fn part_2(races: &Races) -> usize {
    let race = get_p2_input(races);
    race.num_winning_moves()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let races = parse(input);
        let result: usize = races.iter().map(|r| r.num_winning_moves()).product();
        assert_eq!(result, 288);
    }

    #[test]
    fn test_part_2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let races = parse(input);
        assert_eq!(part_2(&races), 71503);
    }
}
