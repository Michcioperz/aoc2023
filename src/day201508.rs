use crate::prelude::*;

pub struct Day201508;
impl TaskA for Day201508 {
    fn solve_a(&self) -> Result<String> {
        Ok(self
            .input()
            .lines()
            .map(|line| {
                let mut it = line.chars();
                let mut len = 0usize;
                let Some('"') = it.next() else {
                    unimplemented!()
                };
                while let Some(c) = it.next() {
                    match c {
                        '"' => break,
                        '\\' => {
                            if it.next().unwrap() == 'x' {
                                _ = it.next().unwrap();
                                _ = it.next().unwrap();
                            }
                        }
                        _ => (),
                    }
                    len += 1;
                }
                line.len() - len
            })
            .sum::<usize>()
            .to_string())
    }
}
impl TaskB for Day201508 {
    fn solve_b(&self) -> Result<String> {
        Ok(self
            .input()
            .lines()
            .map(|line| {
                let mut len = 2usize;
                for c in line.chars() {
                    len += if c == '\\' || c == '"' { 2 } else { 1 };
                }
                len - line.len()
            })
            .sum::<usize>()
            .to_string())
    }
}
