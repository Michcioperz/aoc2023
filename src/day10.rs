use std::{cmp::Ordering, collections::HashMap};

use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Pipe {
    NorthSouth,
    WestEast,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}
impl From<Pipe> for char {
    fn from(value: Pipe) -> Self {
        use Pipe::*;
        match value {
            NorthSouth => '|',
            WestEast => '-',
            NorthEast => 'L',
            NorthWest => 'J',
            SouthWest => '7',
            SouthEast => 'F',
            Ground => '.',
            Start => 'S',
            _ => todo!(),
        }
    }
}
impl From<char> for Pipe {
    fn from(value: char) -> Self {
        use Pipe::*;
        match value {
            '|' => NorthSouth,
            '-' => WestEast,
            'L' => NorthEast,
            'J' => NorthWest,
            '7' => SouthWest,
            'F' => SouthEast,
            '.' => Ground,
            'S' => Start,
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Occupancy {
    Loop,
    Left,
    Right,
    Unknown,
}

pub struct Day10;
impl Day10 {
    fn parsed_input(&self) -> Vec<Vec<Pipe>> {
        self.input()
            .lines()
            .map(|line| line.chars().map(Into::into).collect())
            .collect()
    }
    fn neighbour_candidates(&self, pipe: Pipe, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut res: Vec<(usize, usize)> = Vec::new();
        if let (Pipe::NorthEast | Pipe::NorthSouth | Pipe::NorthWest | Pipe::Start, Some(ym1)) =
            (pipe, y.checked_sub(1))
        {
            res.push((x, ym1));
        }
        if let Pipe::NorthSouth | Pipe::SouthEast | Pipe::SouthWest | Pipe::Start = pipe {
            res.push((x, y + 1));
        }
        if let (Pipe::WestEast | Pipe::NorthWest | Pipe::SouthWest | Pipe::Start, Some(xm1)) =
            (pipe, x.checked_sub(1))
        {
            res.push((xm1, y));
        }
        if let Pipe::WestEast | Pipe::NorthEast | Pipe::SouthEast | Pipe::Start = pipe {
            res.push((x + 1, y));
        }
        res
    }
    fn loop_tiles(&self, map: Vec<Vec<Pipe>>) -> impl Iterator<Item = (usize, usize)> + '_ {
        let start_pos = map
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, pipe)| {
                    if let Pipe::Start = pipe {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .unwrap();
        let mut pos = start_pos;
        let mut previous_pos = start_pos;
        std::iter::from_fn(move || {
            if pos == start_pos && previous_pos != start_pos {
                None
            } else {
                let pipe = map[pos.1][pos.0];
                (previous_pos, pos) = (
                    pos,
                    if let Pipe::Start = pipe {
                        self.neighbour_candidates(pipe, pos)
                            .into_iter()
                            .find(|&next_pos| match map[next_pos.1][next_pos.0] {
                                Pipe::SouthEast | Pipe::SouthWest | Pipe::NorthSouth
                                    if pos.1 == next_pos.1 + 1 =>
                                {
                                    true
                                }
                                Pipe::WestEast | Pipe::NorthEast | Pipe::SouthEast
                                    if pos.0 == next_pos.0 + 1 =>
                                {
                                    true
                                }
                                Pipe::NorthWest | Pipe::NorthEast | Pipe::NorthSouth
                                    if Some(pos.1) == next_pos.1.checked_sub(1) =>
                                {
                                    true
                                }
                                Pipe::WestEast | Pipe::NorthWest | Pipe::SouthWest
                                    if Some(pos.0) == next_pos.0.checked_sub(1) =>
                                {
                                    true
                                }
                                _ => false,
                            })
                            .unwrap()
                    } else {
                        self.neighbour_candidates(pipe, pos)
                            .into_iter()
                            .find(|next_pos| *next_pos != previous_pos)
                            .unwrap()
                    },
                );
                Some(previous_pos)
            }
        })
    }

    fn spread(&self, map: &mut Vec<Vec<Occupancy>>, pos: (usize, usize), target: Occupancy) {
        let Some(cell) = map.get_mut(pos.1).and_then(|row| row.get_mut(pos.0)) else {
            return;
        };
        match (target, *cell) {
            (Occupancy::Left, Occupancy::Right) => todo!(),
            (Occupancy::Right, Occupancy::Left) => todo!(),
            (_, Occupancy::Unknown) => {
                *cell = target;
                for neighbour_pos in [
                    pos.0.checked_sub(1).map(|x| (x, pos.1)),
                    Some((pos.0 + 1, pos.1)),
                    pos.1.checked_sub(1).map(|y| (pos.0, y)),
                    Some((pos.0, pos.1 + 1)),
                ] {
                    if let Some(neighbour_pos) = neighbour_pos {
                        self.spread(map, neighbour_pos, target);
                    }
                }
            }
            _ => (),
        }
    }
}
impl TaskA for Day10 {
    fn solve_a(&self) -> Result<String> {
        let map = self.parsed_input();
        let path_length = self.loop_tiles(map).count();
        Ok((path_length / 2 + path_length % 1).to_string())
    }
}
impl TaskB for Day10 {
    fn solve_b(&self) -> Result<String> {
        let orig_map = self.parsed_input();
        let mut map = vec![vec![Occupancy::Unknown; orig_map[0].len()]; orig_map.len()];
        for tile in self.loop_tiles(orig_map.clone()) {
            map[tile.1][tile.0] = Occupancy::Loop;
        }
        let mut lt = self.loop_tiles(orig_map.clone());
        let mut previous_pos = lt.next().unwrap();
        for pos in lt.chain([previous_pos.clone()]) {
            let diff = (pos.0.cmp(&previous_pos.0), pos.1.cmp(&previous_pos.1));
            for pos in [pos, previous_pos] {
                let (left_pos, right_pos) = match diff {
                    (Ordering::Greater, _) => (
                        pos.1.checked_sub(1).map(|y| (pos.0, y)),
                        Some((pos.0, pos.1 + 1)),
                    ),
                    (Ordering::Less, _) => (
                        Some((pos.0, pos.1 + 1)),
                        pos.1.checked_sub(1).map(|y| (pos.0, y)),
                    ),
                    (_, Ordering::Greater) => (
                        Some((pos.0 + 1, pos.1)),
                        pos.0.checked_sub(1).map(|x| (x, pos.1)),
                    ),
                    (_, Ordering::Less) => (
                        pos.0.checked_sub(1).map(|x| (x, pos.1)),
                        Some((pos.0 + 1, pos.1)),
                    ),
                    _ => unimplemented!(),
                };
                if let Some(left_pos) = left_pos {
                    self.spread(&mut map, left_pos, Occupancy::Left);
                }
                if let Some(right_pos) = right_pos {
                    self.spread(&mut map, right_pos, Occupancy::Right);
                }
            }
            previous_pos = pos;
        }
        Ok(format!(
            "{:?}",
            map.into_iter().fold(HashMap::new(), |mut counts, row| {
                for cell in row {
                    *counts.entry(cell).or_insert(0usize) += 1;
                }
                counts
            })
        ))
        // Ok(map
        //     .into_iter()
        //     .enumerate()
        //     .fold(String::new(), |mut s, (y, row)| {
        //         for (x, cell) in row.into_iter().enumerate() {
        //             s.push(match cell {
        //                 Occupancy::Loop => orig_map[y][x].into(),
        //                 Occupancy::Left => ' ',
        //                 Occupancy::Right => 'X',
        //                 Occupancy::Unknown => '?',
        //             });
        //         }
        //         s.push('\n');
        //         s
        //     }))
    }
}
