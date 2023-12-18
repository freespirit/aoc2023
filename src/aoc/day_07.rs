use std::collections::HashMap;

use crate::day_07::HandType::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
struct Cards([char; 5]);

fn card_rank(card: char) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0,
    }
}
fn card_rank_wildcard(card: char) -> usize {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => 0,
    }
}


impl TryFrom<Vec<char>> for Cards {
    type Error = &'static str;

    fn try_from(value: Vec<char>) -> Result<Self, Self::Error> {
        if value.len() == 5 {
            let mut cards = Cards([Default::default(); 5]);
            for (i, item) in value.into_iter().enumerate() {
                cards.0[i] = item;
            }
            Ok(cards)
        } else {
            Err("Vec does not have exactly 5 elements")
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Copy, Clone)]
struct Hand {
    cards: Cards,
    hand_type: HandType,
    uses_wildcard: bool,
}

impl Hand {
    fn new(cards: Cards, j_is_joker: bool) -> Self {
        let hand_type = if j_is_joker {
            type_of_hand_j(&cards)
        } else {
            type_of_hand(&cards)
        };

        Self { cards, hand_type, uses_wildcard: j_is_joker }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ranker = if self.uses_wildcard {
            card_rank_wildcard
        } else {
            card_rank
        };
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => {
                let self_ranks: Vec<usize> = self.cards.0.map(ranker).to_vec();
                let other_ranks: Vec<usize> = other.cards.0.map(ranker).to_vec();
                self_ranks.cmp(&other_ranks)
            }
            other => other,
        }
    }
}

struct Entry {
    hand: Hand,
    bid: i64,
}

fn type_of_hand(cards: &Cards) -> HandType {
    let mut counter = HashMap::new();
    for card in cards.0.iter() {
        *counter.entry(card).or_insert(0) += 1;
    }

    match (counter.len(), counter.values().max()) {
        (1, _) => FiveOfAKind,
        (2, Some(4)) => FourOfAKind,
        (2, Some(3)) => FullHouse,
        (_, Some(3)) => ThreeOfAKind,
        (3, Some(2)) => TwoPair,
        (_, Some(2)) => OnePair,
        _ => HighCard,
    }
}

/// Determine the type of hand with the joker as a wild card. E.g. if there's a pair of 2s and a joker, then it's a three of a kind.
fn type_of_hand_j(cards: &Cards) -> HandType {
    let mut counter = HashMap::new();
    for &card in cards.0.iter() {
        if card == 'J' {
            continue;
        }

        *counter.entry(card).or_insert(0) += 1;
    }
    let num_jokers = cards.0.into_iter().filter(|&card| card == 'J').count();
    let max_count = counter.values().max().unwrap_or(&0) + num_jokers;
    match (counter.len(), max_count) {
        (_, 5) => FiveOfAKind,
        (_, 4) => FourOfAKind,
        (2, 3) => FullHouse,
        (_, 3) => ThreeOfAKind,
        (3, 2) => TwoPair,
        (_, 2) => OnePair,
        _ => HighCard,
    }
}

pub fn solve_part_1(camel_cards: &str) -> i64 {
    let mut entries: Vec<Entry> = camel_cards.lines().map(|line| {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let cards: Cards = parts[0].chars().collect::<Vec<char>>().try_into().unwrap();
        let hand = Hand::new(cards, false);
        let bid = parts[1].parse().unwrap();
        Entry { hand, bid }
    }).collect();

    entries.sort_by_key(|entry| entry.hand);
    calc_winnings(&entries)
}

pub fn solve_part_2(camel_cards: &str) -> i64 {
    let mut entries: Vec<Entry> = camel_cards.lines().map(|line| {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let cards: Cards = parts[0].chars().collect::<Vec<char>>().try_into().unwrap();
        let hand = Hand::new(cards, true);
        let bid = parts[1].parse().unwrap();
        Entry { hand, bid }
    }).collect();

    entries.sort_by_key(|entry| entry.hand);
    calc_winnings(&entries)
}

fn calc_winnings(entries: &[Entry]) -> i64 {
    entries.iter().enumerate()
        .map(|(i, entry)| {
            let bid = entry.bid;
            bid * ((i + 1) as i64)
        }).sum::<i64>()
}

#[cfg(test)]
mod test {
    use crate::day_07::HandType::FourOfAKind;

    use super::*;

    #[test]
    fn test_type_of_hand() {
        let cards = "AAAAA".chars().collect::<Vec<char>>().try_into().unwrap();
        assert_eq!(type_of_hand(&cards), FiveOfAKind);

        let cards = "33332".chars().collect::<Vec<char>>().try_into().unwrap();
        assert_eq!(type_of_hand(&cards), FourOfAKind);

        let cards = "2AAAA".chars().collect::<Vec<char>>().try_into().unwrap();
        assert_eq!(type_of_hand(&cards), FourOfAKind);

        let cards = "77888".chars().collect::<Vec<char>>().try_into().unwrap();
        assert_eq!(type_of_hand(&cards), FullHouse);

        let cards = "77788".chars().collect::<Vec<char>>().try_into().unwrap();
        assert_eq!(type_of_hand(&cards), FullHouse);

        let cards = "32T3K".chars().collect::<Vec<char>>().try_into().unwrap();
        assert_eq!(type_of_hand(&cards), OnePair);

        let cards = "KK677".chars().collect::<Vec<char>>().try_into().unwrap();
        assert_eq!(type_of_hand(&cards), TwoPair);

        let cards = "KTJJT".chars().collect::<Vec<char>>().try_into().unwrap();
        assert_eq!(type_of_hand(&cards), TwoPair);

        let cards = "T55J5".chars().collect::<Vec<char>>().try_into().unwrap();
        assert_eq!(type_of_hand(&cards), ThreeOfAKind);

        let cards = "QQQJA".chars().collect::<Vec<char>>().try_into().unwrap();
        assert_eq!(type_of_hand(&cards), ThreeOfAKind);
    }


    const CAMEL_CARDS: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(CAMEL_CARDS), 6440);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(CAMEL_CARDS), 5905);
    }

    #[test]
    fn test_type_of_hand_j() {
        let cards: Cards = ['3', '3', 'J', '2', '2'].to_vec().try_into().unwrap();
        assert_eq!(type_of_hand_j(&cards), FullHouse);

        let cards: Cards = ['3', '3', 'J', '4', '5'].to_vec().try_into().unwrap();
        assert_eq!(type_of_hand_j(&cards), ThreeOfAKind);

        let cards: Cards = "32T3K".chars().collect::<Vec<char>>().try_into().unwrap();
        assert_eq!(type_of_hand_j(&cards), OnePair);
        let cards: Cards = "T55J5".chars().collect::<Vec<char>>().try_into().unwrap();
        assert_eq!(type_of_hand_j(&cards), FourOfAKind);
        let cards: Cards = "KK677".chars().collect::<Vec<char>>().try_into().unwrap();
        assert_eq!(type_of_hand_j(&cards), TwoPair);
        let cards: Cards = "KTJJT".chars().collect::<Vec<char>>().try_into().unwrap();
        assert_eq!(type_of_hand_j(&cards), FourOfAKind);
        let cards: Cards = "QQQJA".chars().collect::<Vec<char>>().try_into().unwrap();
        assert_eq!(type_of_hand_j(&cards), FourOfAKind);
    }
}