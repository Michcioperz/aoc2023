use crate::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    Blue,
    Red,
    Green,
}

type Input = Vec<(GameId, Vec<Play>)>;
type GameId = usize;
type Play = Vec<(Color, usize)>;

pub(crate) struct Day2;
impl Day2 {
    fn parsed_input(&self) -> Input {
        self.input()
            .lines()
            .map(|line| {
                let (prefix, games) = line.split_once(": ").unwrap();
                let (_, id_s) = prefix.split_once(' ').unwrap();
                let id: usize = id_s.parse().unwrap();
                (
                    id,
                    games
                        .split("; ")
                        .map(|game| {
                            game.split(", ")
                                .map(|pull| {
                                    let (count_s, color_s) = pull.split_once(' ').unwrap();
                                    let count: usize = count_s.parse().unwrap();
                                    let color = match color_s {
                                        "blue" => Color::Blue,
                                        "green" => Color::Green,
                                        "red" => Color::Red,
                                        _ => unimplemented!(),
                                    };
                                    (color, count)
                                })
                                .collect()
                        })
                        .collect(),
                )
            })
            .collect()
    }
}
impl TaskA for Day2 {
    fn solve_a(&self) -> Result<String> {
        let answer: usize = self
            .parsed_input()
            .into_iter()
            .filter_map(|(id, games)| {
                if games.into_iter().all(|game| {
                    game.into_iter().all(|(color, count)| {
                        count
                            <= match color {
                                Color::Blue => 14,
                                Color::Red => 12,
                                Color::Green => 13,
                            }
                    })
                }) {
                    Some(id)
                } else {
                    None
                }
            })
            .sum();
        Ok(answer.to_string())
    }
}
impl TaskB for Day2 {
    fn solve_b(&self) -> Result<String> {
        Ok(self
            .parsed_input()
            .into_iter()
            .map(|(_id, games)| {
                games
                    .into_iter()
                    .fold(HashMap::new(), |gather, game| {
                        game.into_iter().fold(gather, |mut gather, (color, count)| {
                            {
                                let entry: &mut usize = gather.entry(color).or_default();
                                *entry = (*entry).max(count);
                            }
                            gather
                        })
                    })
                    .into_values()
                    .product::<usize>()
            })
            .sum::<usize>()
            .to_string())
    }
}
