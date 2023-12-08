use std::collections::HashMap;
use std::str::FromStr;

use itertools::Itertools;
use strum_macros::{EnumIter, EnumString};

use crate::_07::Hand::{FiveOfKind, FourOfKind, FullHouse, HighCard, OnePair, ThreeOfKind, TwoPair};

#[derive(Debug, EnumString, EnumIter, PartialEq, PartialOrd, Ord, Eq)]
enum Card {
    #[strum(serialize = "J")] _J,
    #[strum(serialize = "2")] _2,
    #[strum(serialize = "3")] _3,
    #[strum(serialize = "4")] _4,
    #[strum(serialize = "5")] _5,
    #[strum(serialize = "6")] _6,
    #[strum(serialize = "7")] _7,
    #[strum(serialize = "8")] _8,
    #[strum(serialize = "9")] _9,
    #[strum(serialize = "T")] _T,
    #[strum(serialize = "Q")] _Q,
    #[strum(serialize = "K")] _K,
    #[strum(serialize = "A")] _A,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Hand {
    HighCard(Card, Card, Card, Card, Card),
    OnePair(Card, Card, Card, Card, Card),
    TwoPair(Card, Card, Card, Card, Card),
    ThreeOfKind(Card, Card, Card, Card, Card),
    FullHouse(Card, Card, Card, Card, Card),
    FourOfKind(Card, Card, Card, Card, Card),
    FiveOfKind(Card, Card, Card, Card, Card),
}

impl Hand {
    fn parse(str: &str) -> Self {
        let card = Self::parse_cards(str);
        let counters = &Self::count_cards(str)[..];
        match counters {
            [1, 1, 1, 1, 1] => HighCard(card.0, card.1, card.2, card.3, card.4),
            [1, 1, 1, 2] => OnePair(card.0, card.1, card.2, card.3, card.4),
            [1, 2, 2] => TwoPair(card.0, card.1, card.2, card.3, card.4),
            [1, 1, 3] => ThreeOfKind(card.0, card.1, card.2, card.3, card.4),
            [1, 4] => FourOfKind(card.0, card.1, card.2, card.3, card.4),
            [2, 3] => FullHouse(card.0, card.1, card.2, card.3, card.4),
            [5] => FiveOfKind(card.0, card.1, card.2, card.3, card.4),
            _ => panic!("Can't parse {} recognized as {:?}", str, counters)
        }
    }

    fn count_cards(str: &str) -> Vec<usize> {
        let mut counters = HashMap::new();
        for char in str.chars() {
            if counters.contains_key(&char) {
                *counters.get_mut(&char).unwrap() += 1;
            } else {
                counters.insert(char, 1);
            }
        }
        counters.values().sorted().copied().collect_vec()
    }

    fn parse_cards(str: &str) -> (Card, Card, Card, Card, Card) {
        (0..str.len())
            .map(|i| &str[i..i + 1])
            .map(Card::from_str)
            .map(Result::unwrap)
            .collect_tuple()
            .unwrap()
    }
}

struct HandBid {
    hand: Hand,
    bid: u128,
}

impl HandBid {
    fn parse(str: &str) -> Self {
        let (hand, bid) = str.split_once(' ').unwrap();
        let hand = Hand::parse(hand);
        let bid = bid.parse::<u128>().unwrap();
        Self { hand, bid }
    }
}

#[test]
fn part_one() {
    let answer =
        include_str!("../07.input")
            .trim()
            .lines()
            .map(|s| HandBid::parse(s))
            .sorted_by(|a, b| Ord::cmp(&a.hand, &b.hand))
            .map(|x| x.bid)
            .enumerate()
            .map(|(pos, bid)| bid * (pos + 1) as u128)
            .sum::<u128>();
    println!("{:?}", answer);
}