use crate::DayResult;
use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let games: Vec<GameResults> = parse_games(input);
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let p1 = part_1(&games).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&games).to_string();
    let p2_duration = start.elapsed();
    (Some(parse_duration), (p1, p1_duration), (p2, p2_duration))
}

#[derive(Default, Copy, Clone, Debug)]
struct CubeCounts {
    red: usize,
    blue: usize,
    green: usize,
}

#[derive(Default, Clone, Debug)]
struct GameResults {
    id: usize,
    counts: Vec<CubeCounts>,
}

impl GameResults {
    fn get_max_counts(&self) -> CubeCounts {
        let red = self.counts.iter().max_by_key(|c| c.red).unwrap().red;
        let blue = self.counts.iter().max_by_key(|c| c.blue).unwrap().blue;
        let green = self.counts.iter().max_by_key(|c| c.green).unwrap().green;
        CubeCounts { red, blue, green }
    }
}

impl GameResults {
    fn new(input: &str) -> Self {
        let mut fields = input.split(": ");
        let id = fields
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut counts: Vec<CubeCounts> = Vec::new();
        for run in fields.next().unwrap().split("; ") {
            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;
            for entry in run.split(", ") {
                let mut values = entry.split(' ');
                let count = values.next().unwrap().parse::<usize>().unwrap();
                match values.next().unwrap() {
                    "red" => red += count,
                    "blue" => blue += count,
                    "green" => green += count,
                    _ => unreachable!("Unknown cube color!"),
                }
            }
            counts.push(CubeCounts { red, blue, green });
        }
        Self { id, counts }
    }
}

fn parse_games(input: &str) -> Vec<GameResults> {
    let mut games: Vec<GameResults> = Vec::new();
    for line in input.lines() {
        games.push(GameResults::new(line));
    }
    games
}

#[inline(always)]
fn part_1(games: &Vec<GameResults>) -> usize {
    // Rules given by the prompt
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    // Count the games that have valid cube counts
    let mut sum_of_valid_game_ids = 0;
    for g in games {
        let counts = g.get_max_counts();
        if counts.red <= max_red && counts.blue <= max_blue && counts.green <= max_green {
            sum_of_valid_game_ids += g.id;
        }
    }
    sum_of_valid_game_ids
}

#[inline(always)]
fn part_2(games: &Vec<GameResults>) -> usize {
    let mut power_sum = 0;
    for g in games {
        let counts = g.get_max_counts();
        power_sum += counts.red * counts.green * counts.blue;
    }
    power_sum
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part_1() {
        let result = part_1(&parse_games(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ));
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(&parse_games(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ));
        assert_eq!(result, 2286);
    }
}
