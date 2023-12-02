use std::sync::OnceLock;

use clap::ValueEnum;
use color_eyre::Result;
use regex::Regex;

pub static CUSTOM_INPUT: OnceLock<String> = OnceLock::new();

struct Day1;
impl TaskA for Day1 {
    fn solve_a(&self) -> Result<String> {
        let answer: u32 = self
            .input()
            .lines()
            .map(|line| {
                let mut digits = line.chars().filter_map(|c| c.to_digit(10));
                let first = digits.next().unwrap();
                let last = digits.last().unwrap_or(first);
                first * 10 + last
            })
            .sum();
        Ok(answer.to_string())
    }
}
impl TaskB for Day1 {
    fn solve_b(&self) -> Result<String> {
        let digit_regex = Regex::new(r"(?:[1-9]|one|two|three|four|five|six|seven|eight|nine)")?;
        let reverse_digit_regex =
            Regex::new(r"(?:[1-9]|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno)")?;
        fn parse_digit(s: &str) -> u32 {
            match s {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                s => s.chars().next().unwrap().to_digit(10).unwrap(),
            }
        }
        let answer: u32 = self
            .input()
            .lines()
            .map(|line| {
                let first = parse_digit(digit_regex.find(line).unwrap().as_str());
                let reversed = line.chars().rev().collect::<String>();
                let last = parse_digit(
                    &reverse_digit_regex
                        .find(&reversed)
                        .unwrap()
                        .as_str()
                        .chars()
                        .rev()
                        .collect::<String>(),
                );
                first * 10 + last
            })
            .sum();
        Ok(answer.to_string())
    }
}

use day2::Day2;
mod day2;

include!(concat!(env!("OUT_DIR"), "/inputs.rs"));

pub trait DayInput {
    const CONTENTS: &'static str;
}

pub trait TaskA: DayInput {
    /// Helper function for Self::CONTENTS, because rust-analyzer no likey somehow.
    fn input(&self) -> &str {
        if let Some(custom_input) = CUSTOM_INPUT.get() {
            custom_input.as_str()
        } else {
            Self::CONTENTS
        }
    }

    fn solve_a(&self) -> Result<String> {
        todo!()
    }
}

pub trait TaskB: TaskA {
    fn solve_b(&self) -> Result<String> {
        todo!()
    }
}
