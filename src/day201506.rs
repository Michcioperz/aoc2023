use crate::prelude::*;

enum Instruction {
    On,
    Off,
    Toggle,
}

pub struct Day201506;
impl Day201506 {
    fn parsed_input(
        &self,
    ) -> impl Iterator<Item = (Instruction, (usize, usize), (usize, usize))> + '_ {
        self.input().lines().map(|line| {
            let mut it = line.split_whitespace();
            let instruction = match it.next().unwrap() {
                "turn" => match it.next().unwrap() {
                    "on" => Instruction::On,
                    "off" => Instruction::Off,
                    _ => unimplemented!(),
                },
                "toggle" => Instruction::Toggle,
                _ => unimplemented!(),
            };
            let from = it
                .next()
                .unwrap()
                .split_once(',')
                .unwrap()
                .via_iter(|it| it.map(|i| i.parse().unwrap()));
            let to = it
                .last()
                .unwrap()
                .split_once(',')
                .unwrap()
                .via_iter(|it| it.map(|i| i.parse().unwrap()));
            (instruction, from, to)
        })
    }
}

impl TaskA for Day201506 {
    fn solve_a(&self) -> Result<String> {
        let mut grid = [[false; 1000]; 1000];
        for (instr, (from_i, from_j), (to_i, to_j)) in self.parsed_input() {
            let op = match instr {
                Instruction::On => |b: &mut bool| *b = true,
                Instruction::Off => |b: &mut bool| *b = false,
                Instruction::Toggle => |b: &mut bool| *b = !*b,
            };
            for light in grid
                .get_mut(from_i..=to_i)
                .unwrap()
                .iter_mut()
                .flat_map(|row| row.get_mut(from_j..=to_j).unwrap())
            {
                op(light);
            }
        }
        Ok(grid
            .into_iter()
            .map(|row| row.into_iter().filter(|b| *b).count())
            .sum::<usize>()
            .to_string())
    }
}
impl TaskB for Day201506 {
    fn solve_b(&self) -> Result<String> {
        let mut grid = vec![[0usize; 1000]; 1000];
        for (instr, (from_i, from_j), (to_i, to_j)) in self.parsed_input() {
            let op = match instr {
                Instruction::On => |b: &mut usize| *b += 1,
                Instruction::Off => |b: &mut usize| *b = b.saturating_sub(1),
                Instruction::Toggle => |b: &mut usize| *b += 2,
            };
            for light in grid
                .get_mut(from_i..=to_i)
                .unwrap()
                .iter_mut()
                .flat_map(|row| row.get_mut(from_j..=to_j).unwrap())
            {
                op(light);
            }
        }
        Ok(grid
            .into_iter()
            .map(|row| row.into_iter().sum::<usize>())
            .sum::<usize>()
            .to_string())
    }
}
