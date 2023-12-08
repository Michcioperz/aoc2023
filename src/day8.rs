use std::collections::HashMap;

use num_integer::Integer;

use crate::prelude::*;

pub struct Day8;
impl Day8 {
    fn parsed_input(
        &self,
    ) -> (
        &'static str,
        HashMap<&'static str, (&'static str, &'static str)>,
    ) {
        let mut lines = self.input().lines();
        let directions = lines.next().unwrap();
        _ = lines.next().unwrap();
        (
            directions,
            lines
                .map(|line| {
                    line.split_once(" = ")
                        .map(|(key, value)| {
                            (
                                key,
                                value
                                    .trim_start_matches('(')
                                    .trim_end_matches(')')
                                    .split_once(", ")
                                    .unwrap(),
                            )
                        })
                        .unwrap()
                })
                .collect(),
        )
    }
    fn subsolution(
        &self,
        directions: &str,
        map: &HashMap<&str, (&str, &str)>,
        start: &str,
        all_z: bool,
    ) -> usize {
        let cycle_length = directions.len();
        let mut directions = directions.chars().cycle();
        let mut current = start;
        let mut steps = 0usize;
        while !current.ends_with('Z') || (all_z && current != "ZZZ") {
            let node = map[current];
            current = match directions.next().unwrap() {
                'L' => node.0,
                'R' => node.1,
                _ => unreachable!(),
            };
            steps += 1;
        }
        assert!(steps % cycle_length == 0);
        steps
    }
}
impl TaskA for Day8 {
    fn solve_a(&self) -> Result<String> {
        let (directions, map) = self.parsed_input();

        Ok(self.subsolution(directions, &map, "AAA", true).to_string())
    }
}
impl TaskB for Day8 {
    fn solve_b(&self) -> Result<String> {
        let (directions, map) = self.parsed_input();
        let subsolutions = map
            .keys()
            .copied()
            .filter(|node| node.ends_with('A'))
            .map(|start| self.subsolution(directions, &map, start, false))
            .collect_vec();
        let gcd = subsolutions
            .iter()
            .copied()
            .reduce(|a, b| a.gcd(&b))
            .unwrap();
        let solution = subsolutions
            .into_iter()
            .reduce(|a, b| a * (b / gcd))
            .unwrap();
        assert!(solution % directions.len() == 0);
        Ok(solution.to_string())
    }
}
