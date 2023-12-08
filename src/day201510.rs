use crate::prelude::*;

pub struct Day201510;
impl Day201510 {
    fn step(&self, s: &str) -> String {
        s.chars()
            .group_by(|x| *x)
            .into_iter()
            .map(|(digit, group)| format!("{}{}", group.count(), digit))
            .collect()
    }
}
impl TaskA for Day201510 {
    fn solve_a(&self) -> Result<String> {
        let mut s = self.input().to_string();
        for _ in 0..40 {
            s = self.step(&s);
        }
        Ok(s.len().to_string())
    }
}
impl TaskB for Day201510 {
    fn solve_b(&self) -> Result<String> {
        let mut s = self.input().to_string();
        for _ in 0..50 {
            s = self.step(&s);
        }
        Ok(s.len().to_string())
    }
}
