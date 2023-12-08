use crate::day07::Card::{
    Ace, Eight, Five, Four, Jack, Joker, King, Nine, Queen, Seven, Six, Ten, Three, Two,
};
use crate::day07::HandKind::{
    FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair,
};
use crate::DayResult;
use fxhash::FxHashMap;
use std::cmp::Ordering;

use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let mut hands: Vec<Hand> = input.lines().map(Hand::new).collect();
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let p1 = part_1(&mut hands).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&mut hands).to_string();
    let p2_duration = start.elapsed();
    (Some(parse_duration), (p1, p1_duration), (p2, p2_duration))
    // (None, (p1, p1_duration), (p2, p2_duration))
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}

impl Card {
    fn new(input: char) -> Self {
        match input {
            'A' => Ace,
            'K' => King,
            'Q' => Queen,
            'J' => Jack,
            'T' => Ten,
            '9' => Nine,
            '8' => Eight,
            '7' => Seven,
            '6' => Six,
            '5' => Five,
            '4' => Four,
            '3' => Three,
            '2' => Two,
            _ => unreachable!("Unknown card"),
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum HandKind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandKind {
    fn from_cards(cards: &[Card]) -> Self {
        let mut freq = FxHashMap::default();
        for &c in cards {
            *freq.entry(c).or_insert(0) += 1;
        }
        let values = freq.into_values().collect::<Vec<usize>>();
        let num_pairs = values.iter().fold(0, |acc, v| acc + (v == &2) as usize);

        if values.contains(&5) {
            return FiveOfAKind;
        } else if values.contains(&4) {
            return FourOfAKind;
        } else if values.contains(&3) && num_pairs == 1 {
            return FullHouse;
        } else if values.contains(&3) {
            return ThreeOfAKind;
        };

        match num_pairs {
            2 => TwoPair,
            1 => OnePair,
            _ => HighCard,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
    kind: HandKind,
}

impl Hand {
    fn new(input: &str) -> Self {
        let mut fields = input.split_ascii_whitespace();
        let cards = fields
            .next()
            .unwrap()
            .chars()
            .map(Card::new)
            .collect::<Vec<Card>>();
        let bid = fields.next().unwrap().parse::<usize>().unwrap();
        let kind = HandKind::from_cards(&cards);
        Self { cards, bid, kind }
    }

    /// Jokers take the place of the card with the highest frequency and highest value if there is
    /// a tie. So `2TJT2` would turn into `2TTT2` with these rules. The case of `oops, all jokers`
    /// just results in a FiveOfAKind with cards valued at `1` each, as Jokers.
    ///
    /// WARNING: J is always treated as J, not the card it's pretending to be!
    fn jokerize(&mut self) {
        let mut freq = FxHashMap::default();
        for c in &self.cards {
            *freq.entry(c).or_insert(0) += 1;
        }
        if let Some(5) = freq.get(&Jack) {
            for c in &mut self.cards {
                *c = Joker;
                self.kind = HandKind::FiveOfAKind;
            }
        } else {
            let mut tmp_cards = self.cards.clone();
            let mut values = freq
                .clone()
                .iter()
                .map(|(&&k, &v)| (k, v))
                .filter(|(k, _)| *k != Jack)
                .collect::<Vec<_>>();
            values.sort_by_key(|(card, num)| (-(*num), -(*card as isize)));

            for c in &mut tmp_cards {
                if *c == Jack {
                    *c = values.first().unwrap().0;
                }
            }
            self.kind = HandKind::from_cards(&tmp_cards);

            for c in &mut self.cards {
                if *c == Jack {
                    *c = Joker;
                }
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.kind)
            .cmp(&self.kind)
            .then(self.cards.cmp(&other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part_1(input: &mut [Hand]) -> usize {
    input.sort();
    let mut result = 0;
    for (rank, hand) in input.iter().enumerate() {
        result += hand.bid * (rank + 1);
    }
    result
}

fn part_2(input: &mut [Hand]) -> usize {
    let mut result = 0;
    for hand in input.iter_mut() {
        hand.jokerize();
    }
    input.sort();
    for (rank, hand) in input.iter_mut().enumerate() {
        result += hand.bid * (rank + 1);
    }
    result
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let mut hands: Vec<Hand> = input.lines().map(Hand::new).collect();
        assert_eq!(part_1(&mut hands), 6440);
    }

    #[test]
    fn test_part_2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let mut hands: Vec<Hand> = input.lines().map(Hand::new).collect();
        hands.sort();
        assert_eq!(part_2(&mut hands), 5905);
    }
}
