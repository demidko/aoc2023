use std::collections::HashSet;

use itertools::Itertools;

#[test]
fn part_one() {
    let answer =
        include_str!("../04.input")
            .trim()
            .lines()
            .map(|x| Card::parse(x).calculate_card_scores_for_part_one())
            .sum::<u32>();
    println!("Day 4: Scratchcards is {}", answer)
}

#[test]
fn part_two() {
    let original_cards =
        include_str!("../04.input")
            .trim()
            .lines()
            .map(Card::parse)
            .collect_vec();
    let original_len = original_cards.len();
    let mut copy_processor = CopyProcessor::new(original_cards);
    for i in 0..original_len {
        copy_processor.process_card(i);
    }
    println!("Part Two is {}", copy_processor.count_copies())
}

struct CopyProcessor {
    cards: Vec<usize>,
    copies: Vec<usize>,
}

impl CopyProcessor {
    fn new(cards: Vec<Card>) -> Self {
        let cards =
            cards.iter()
                .map(Card::free_cards)
                .collect::<Vec<usize>>();
        let copies =
            (0..cards.len())
                .map(|_| 1)
                .collect_vec();
        Self { cards, copies }
    }

    fn process_card(&mut self, id: usize) {
        let self_copies = self.copies[id];
        for _ in 0..self_copies {
            let next_copies = self.cards[id];
            if next_copies > 0 {
                for id in id + 1..id + 1 + next_copies {
                    self.copies[id] += 1;
                }
            }
        }
    }

    fn count_copies(&self) -> usize {
        self.copies.iter().sum()
    }
}

#[derive(Clone)]
struct Card {
    actual_numbers: HashSet<u32>,
    winning_numbers: HashSet<u32>,
}

impl Card {
    fn parse(card: &str) -> Self {
        let (_, numbers) = card.split_once(": ").unwrap();
        let (actual_numbers, winning_numbers) = numbers.split_once(" | ").unwrap();
        let actual_numbers = parse_uniq_numbers(actual_numbers);
        let winning_numbers = parse_uniq_numbers(winning_numbers);
        Self { actual_numbers, winning_numbers }
    }

    fn free_cards(&self) -> usize {
        self.actual_numbers.intersection(&self.winning_numbers).count()
    }

    fn list_actual_winners(&self) -> Vec<u32> {
        self.actual_numbers
            .intersection(&self.winning_numbers)
            .copied()
            .collect_vec()
    }

    fn calculate_card_scores_for_part_one(&self) -> u32 {
        let actual_winners = self.list_actual_winners();
        if actual_winners.is_empty() {
            return 0;
        }
        let mut scores = 1;
        for _ in actual_winners.iter().skip(1) {
            scores *= 2
        }
        scores
    }
}

fn parse_uniq_numbers(str: &str) -> HashSet<u32> {
    str.split_whitespace()
        .map(|x| x.parse::<u32>())
        .filter_map(Result::ok)
        .collect()
}