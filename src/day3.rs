use std::collections::{BTreeMap, BTreeSet};

use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Number {
    x: usize,
    y: usize,
    len: usize,
    value: u64,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Symbol {
    x: usize,
    y: usize,
    is_gear: bool,
}

fn adjacent(symbol: &Symbol, number: &Number) -> bool {
    (number.x.saturating_sub(1)..=number.x.saturating_add(number.len)).contains(&symbol.x)
        && (number.y.saturating_sub(1)..=number.y.saturating_add(1)).contains(&symbol.y)
}

pub struct Day3;
impl Day3 {
    fn symbols(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.input().lines().enumerate().flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, ch)| {
                if ch != '.' && !ch.is_numeric() {
                    Some(Symbol {
                        x,
                        y,
                        is_gear: ch == '*',
                    })
                } else {
                    None
                }
            })
        })
    }
    fn numbers(&self) -> impl Iterator<Item = Number> + '_ {
        self.input().lines().enumerate().flat_map(|(y, line)| {
            let (last, mut stash) = line.chars().enumerate().fold(
                (None, Vec::new()),
                |(current, mut stash), (x, ch)| {
                    if let Some(digit) = ch.to_digit(10) {
                        (
                            Some(current.map_or(
                                Number {
                                    x,
                                    y,
                                    len: 1,
                                    value: u64::from(digit),
                                },
                                |number: Number| Number {
                                    len: number.len + 1,
                                    value: number.value * 10 + u64::from(digit),
                                    ..number
                                },
                            )),
                            stash,
                        )
                    } else {
                        if let Some(number) = current {
                            stash.push(number);
                        }
                        (None, stash)
                    }
                },
            );
            if let Some(last) = last {
                stash.push(last);
            }
            stash
        })
    }
}
impl TaskA for Day3 {
    fn solve_a(&self) -> Result<String> {
        let symbols: BTreeSet<Symbol> = self.symbols().collect();
        Ok(self
            .numbers()
            .filter_map(|number| {
                if symbols.iter().any(|symbol| adjacent(symbol, &number)) {
                    Some(number.value)
                } else {
                    None
                }
            })
            .sum::<u64>()
            .to_string())
    }
}
impl TaskB for Day3 {
    fn solve_b(&self) -> Result<String> {
        let mut symbols: BTreeMap<Symbol, Vec<u64>> =
            self.symbols().map(|symbol| (symbol, Vec::new())).collect();
        for number in self.numbers() {
            for (symbol, ratios) in symbols.iter_mut() {
                if adjacent(symbol, &number) {
                    ratios.push(number.value);
                }
            }
        }
        Ok(symbols
            .into_iter()
            .filter_map(|(_, ratios)| {
                if ratios.len() == 2 {
                    Some(ratios.into_iter().product::<u64>())
                } else {
                    None
                }
            })
            .sum::<u64>()
            .to_string())
    }
}
