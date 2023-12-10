use std::collections::HashMap;

use crate::prelude::*;

pub struct Day201513;
impl Day201513 {
    fn parsed_input(&self) -> HashMap<&'static str, HashMap<&'static str, isize>> {
        let mut map = HashMap::new();
        for line in self.input().lines() {
            // let (source, rest) = 
        }
        map
    }
}
impl TaskA for Day201513 {
    fn solve_a(&self) -> color_eyre::eyre::Result<String> {
        todo!()
    }
}
impl TaskB for Day201513 {}