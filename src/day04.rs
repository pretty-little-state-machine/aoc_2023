use crate::DayResult;
use fxhash::{FxHashMap, FxHashSet};
use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let cards: Vec<Scratchcard> = input.lines().map(Scratchcard::new).collect();
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let p1 = part_1(&cards).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&cards).to_string();
    let p2_duration = start.elapsed();
    (Some(parse_duration), (p1, p1_duration), (p2, p2_duration))
}

#[derive(Debug, Clone)]
struct Scratchcard {
    id: usize,
    revealed_numbers: FxHashSet<usize>,
    winning_numbers: FxHashSet<usize>,
}

impl Scratchcard {
    fn new(input: &str) -> Self {
        let fields: Vec<&str> = input.split(|c: char| c.is_ascii_punctuation()).collect();
        Self {
            id: fields
                .first()
                .unwrap()
                .split(' ')
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            revealed_numbers: num_columns_to_set(fields.get(1).unwrap().trim()),
            winning_numbers: num_columns_to_set(fields.get(2).unwrap().trim()),
        }
    }

    fn get_worth(&self) -> usize {
        let matches: Vec<&usize> = self
            .revealed_numbers
            .intersection(&self.winning_numbers)
            .collect();
        if !matches.is_empty() {
            (2_u32.pow(matches.len() as u32) / 2) as usize
        } else {
            0
        }
    }

    fn num_matches(&self) -> usize {
        self.revealed_numbers
            .intersection(&self.winning_numbers)
            .collect::<Vec<&usize>>()
            .len()
    }
}

#[inline(always)]
fn num_columns_to_set(input: &str) -> FxHashSet<usize> {
    let mut result = FxHashSet::default();
    let mut value = 0;
    let mut offset = 1;
    for char in input.chars().rev() {
        if let Some(number) = char.to_digit(10) {
            value += number as usize * offset;
            offset *= 10;
        } else {
            result.insert(value);
            value = 0;
            offset = 1;
        }
    }
    result.insert(value);
    result.remove(&0);
    result
}

fn part_1(cards: &[Scratchcard]) -> usize {
    cards.iter().map(|c| c.get_worth()).sum()
}

fn part_2(cards: &[Scratchcard]) -> usize {
    let mut cards_to_run: FxHashMap<usize, usize> = FxHashMap::default();
    for card in cards {
        let mut multiplier = 1;
        if let Some(&x) = cards_to_run.get(&card.id) {
            multiplier = x;
        } else {
            cards_to_run.insert(card.id, 1);
        }

        // Add additional card iterations
        for n in card.id + 1..=card.id + card.num_matches() {
            if let Some(v) = cards_to_run.get_mut(&n) {
                *v += multiplier;
            } else {
                cards_to_run.insert(n, multiplier + 1);
            }
        }
    }
    cards_to_run.values().sum::<usize>()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        let cards: Vec<Scratchcard> = input.lines().map(Scratchcard::new).collect();
        assert_eq!(part_1(&cards), 13);
    }

    #[test]
    fn test_part_2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        let cards: Vec<Scratchcard> = input.lines().map(Scratchcard::new).collect();
        assert_eq!(part_2(&cards), 30);
    }
}
