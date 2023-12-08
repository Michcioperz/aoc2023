use crate::prelude::*;

pub struct Day6;
impl Day6 {
    fn parsed_input(&self) -> Vec<(usize, usize)> {
        let mut lines = self.input().lines();
        let times = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap());
        let records = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap());
        times.zip(records).collect()
    }
}
impl TaskA for Day6 {
    fn solve_a(&self) -> Result<String> {
        Ok(self
            .parsed_input()
            .into_iter()
            .map(|(time, record)| {
                (0..=time)
                    .map(|accel_time| accel_time * (time - accel_time))
                    .filter(|distance| distance > &record)
                    .count()
            })
            .product::<usize>()
            .to_string())
    }
}
impl TaskB for Day6 {
    fn solve_b(&self) -> Result<String> {
        let mut lines = self.input().lines();
        let time = lines.next().unwrap().chars().fold(0, |acc, ch| {
            if let Some(digit) = ch.to_digit(10) {
                acc * 10 + digit as usize
            } else {
                acc
            }
        });
        let record = lines.next().unwrap().chars().fold(0, |acc, ch| {
            if let Some(digit) = ch.to_digit(10) {
                acc * 10 + digit as usize
            } else {
                acc
            }
        });
        Ok((0..=time)
            .map(|accel_time| accel_time * (time - accel_time))
            .filter(|distance| distance > &record)
            .count()
            .to_string())
    }
}
