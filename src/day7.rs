use std::collections::HashMap;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
impl Hand {
    fn from_with_joker(mut cards: Vec<u8>) -> Self {
        cards.retain(|card| *card != 1);
        let jokers = 5 - cards.len();
        (0usize..(12usize.pow(jokers.try_into().unwrap())))
            .map(|mut combo| {
                let mut cards = cards.clone();
                for _joker in 0..jokers {
                    cards.push({
                        let c = combo % 12;
                        match c {
                            0..=8 => c + 2,
                            9 => 12,
                            10 => 13,
                            11 => 14,
                            _ => unreachable!(),
                        }
                        .try_into()
                        .unwrap()
                    });
                    combo /= 12;
                }
                Hand::from(cards.as_slice())
            })
            .max()
            .unwrap()
    }
}
impl From<&[u8]> for Hand {
    fn from(value: &[u8]) -> Self {
        let mut hand: [u8; 5] = value.try_into().unwrap();
        hand.sort_unstable();
        let counts = hand.iter().fold(HashMap::new(), |mut counts, card| {
            *counts.entry(*card).or_insert(0usize) += 1;
            counts
        });
        if counts.len() == 1 {
            Hand::FiveOfAKind
        } else if counts.values().find(|v| **v == 4).is_some() {
            Hand::FourOfAKind
        } else if counts.len() == 2 {
            Hand::FullHouse
        } else if counts.values().find(|v| **v == 3).is_some() {
            Hand::ThreeOfAKind
        } else if counts.values().filter(|v| **v == 2).count() == 2 {
            Hand::TwoPair
        } else if counts.values().find(|v| **v == 2).is_some() {
            Hand::OnePair
        } else {
            Hand::HighCard
        }
    }
}

type Card = u8;
const fn parse_card(c: char) -> Card {
    match c {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => {
            if let Some(d) = c.to_digit(10) {
                d as u8
            } else {
                unreachable!()
            }
        }
    }
}

type Bid = usize;

pub struct Day7;
impl Day7 {
    fn parsed_input(&self) -> Vec<(Vec<Card>, Bid)> {
        self.input()
            .lines()
            .map(|line| {
                let (hand, bid) = line.split_once(' ').unwrap();
                let hand = hand.chars().map(|c| parse_card(c)).collect();
                (hand, bid.parse().unwrap())
            })
            .collect()
    }
}
impl TaskA for Day7 {
    fn solve_a(&self) -> Result<String> {
        let mut hands: Vec<(Vec<u8>, usize)> = self.parsed_input();
        hands.sort_unstable_by_key(|(hand, _)| (Hand::from(hand.as_slice()), hand.clone()));
        Ok(hands
            .into_iter()
            .enumerate()
            .map(|(i, (_, bid))| (i + 1) * bid)
            .sum::<usize>()
            .to_string())
    }
}
impl TaskB for Day7 {
    fn solve_b(&self) -> Result<String> {
        let mut hands = self.parsed_input();
        for (hand, _) in hands.iter_mut() {
            for card in hand.iter_mut() {
                if *card == parse_card('J') {
                    *card = 1;
                }
            }
        }
        hands.sort_by_cached_key(|(hand, _)| (Hand::from_with_joker(hand.clone()), hand.clone()));
        Ok(hands
            .into_iter()
            .enumerate()
            .map(|(i, (_, bid))| (i + 1) * bid)
            .sum::<usize>()
            .to_string())
    }
}
