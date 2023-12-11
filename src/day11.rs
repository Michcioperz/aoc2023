use std::collections::BTreeSet;

use crate::prelude::*;

struct Parsed {
    galaxies: Vec<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

pub struct Day11;
impl Day11 {
    fn parsed_input(&self) -> Parsed {
        let map: Vec<&str> = self.input().lines().collect();
        let galaxies = map
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.char_indices()
                    .filter_map(move |(x, ch)| if ch == '#' { Some((x, y)) } else { None })
            })
            .collect();
        let empty_rows = map
            .iter()
            .enumerate()
            .filter_map(|(y, line)| if line.contains('#') { None } else { Some(y) })
            .collect();
        let empty_cols = map
            .iter()
            .map(|line| {
                line.char_indices()
                    .filter_map(|(x, ch)| if ch == '.' { Some(x) } else { None })
                    .collect::<BTreeSet<usize>>()
            })
            .reduce(|a, b| a.intersection(&b).copied().collect())
            .unwrap()
            .into_iter()
            .collect();
        Parsed {
            galaxies,
            empty_rows,
            empty_cols,
        }
    }
    fn solve_for_scale(&self, scale: usize) -> usize {
        let parsed = self.parsed_input();
        let scale = scale.checked_sub(1).unwrap();
        parsed
            .galaxies
            .iter()
            .enumerate()
            .flat_map(|(i, source)| {
                parsed
                    .galaxies
                    .iter()
                    .skip(i + 1)
                    .map(move |dest| (*source, *dest))
            })
            .map(move |(source, dest)| {
                let min_x = source.0.min(dest.0);
                let max_x = source.0.max(dest.0);
                let min_y = source.1.min(dest.1);
                let max_y = source.1.max(dest.1);
                let x_diff = max_x - min_x
                    + (parsed.empty_cols.partition_point(|col| col < &max_x)
                        - parsed.empty_cols.partition_point(|col| col < &min_x))
                    .checked_mul(scale)
                    .unwrap();
                let y_diff = max_y - min_y
                    + (parsed.empty_rows.partition_point(|row| row < &max_y)
                        - parsed.empty_rows.partition_point(|row| row < &min_y))
                    .checked_mul(scale)
                    .unwrap();
                x_diff + y_diff
            })
            .sum::<usize>()
    }
}
impl TaskA for Day11 {
    fn solve_a(&self) -> Result<String> {
        Ok(self.solve_for_scale(2).to_string())
    }
}
impl TaskB for Day11 {
    fn solve_b(&self) -> Result<String> {
        Ok(self.solve_for_scale(1000000).to_string())
    }
}
