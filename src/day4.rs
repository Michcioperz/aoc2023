use std::collections::HashSet;

use crate::prelude::*;

struct Card {
    winning: HashSet<u32>,
    yours: Vec<u32>,
}

impl Card {
    fn matching(&self) -> usize {
        self.yours
            .iter()
            .filter(|number| self.winning.contains(number))
            .count()
    }
}

pub struct Day4;
impl Day4 {
    fn parsed_input(&self) -> impl Iterator<Item = Card> + '_ {
        self.input().lines().map(|line| {
            line.split_once(": ")
                .unwrap()
                .1
                .split_once(" | ")
                .map(|(winning, yours)| Card {
                    winning: winning
                        .split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect(),
                    yours: yours
                        .split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect(),
                })
                .unwrap()
        })
    }
}
impl TaskA for Day4 {
    fn solve_a(&self) -> Result<String> {
        Ok(self
            .parsed_input()
            .map(|card| card.matching())
            .map(|number| if number > 0 { 1 << (number - 1) } else { 0 })
            .sum::<u32>()
            .to_string())
    }
}
impl TaskB for Day4 {
    fn solve_b(&self) -> Result<String> {
        Ok(self
            .parsed_input()
            .enumerate()
            .fold(
                vec![1usize; self.parsed_input().count()],
                |mut counts, (i, card)| {
                    let count = *counts.get(i).unwrap();
                    let matching = card.matching();
                    for j in i + 1..=(i + matching).min(counts.len() - 1) {
                        *counts.get_mut(j).unwrap() += count;
                    }
                    counts
                },
            )
            .into_iter()
            .sum::<usize>()
            .to_string())
    }
}
