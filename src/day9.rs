use crate::prelude::*;

pub struct Day9;

impl Day9 {
    fn parsed_input(&self) -> impl Iterator<Item = Vec<isize>> + '_ {
        self.input().lines().map(|line| {
            line.split_whitespace()
                .map(|i| i.parse().unwrap())
                .collect_vec()
        })
    }

    fn predictors(&self) -> impl Iterator<Item = Vec<Vec<isize>>> + '_ {
        self.parsed_input().map(|line| {
            let mut predictors = vec![line];
            while let Some(last) = predictors
                .last()
                .filter(|line| line.iter().any(|v| *v != 0))
            {
                let new_line = last
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| *b - *a)
                    .collect_vec();
                predictors.push(new_line);
            }
            predictors
        })
    }
}
impl TaskA for Day9 {
    fn solve_a(&self) -> Result<String> {
        Ok(self
            .predictors()
            .map(|predictors| {
                predictors
                    .into_iter()
                    .rev()
                    .fold(0isize, |acc, line| line.last().unwrap() + acc)
            })
            .sum::<isize>()
            .to_string())
    }
}
impl TaskB for Day9 {
    fn solve_b(&self) -> Result<String> {
        Ok(self
            .predictors()
            .map(|predictors| {
                predictors
                    .into_iter()
                    .rev()
                    .fold(0isize, |acc, line| line.first().unwrap() - acc)
            })
            .sum::<isize>()
            .to_string())
    }
}
