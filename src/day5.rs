use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    str::FromStr,
};

use crate::prelude::*;

type ConversionTable = HashMap<Type, BTreeSet<Conversion>>;
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Conversion {
    source_start: usize,
    destination_start: usize,
    length: usize,
}
impl Conversion {
    fn try_convert(&self, item: usize) -> Option<usize> {
        if (self.source_start..self.source_start + self.length).contains(&item) {
            Some(item - self.source_start + self.destination_start)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct IntervalConversionTable(BTreeMap<usize, usize>);
impl IntervalConversionTable {
    fn parse<'a>(section_lines: impl Iterator<Item = &'a str>) -> Self {
        let mut entries: Vec<_> = section_lines
            .map(|line| {
                let (destination_start, (source_start, length)) = line
                    .split_once(' ')
                    .map(|(ls, rs)| {
                        (
                            ls.parse().unwrap(),
                            rs.split_once(' ')
                                .map(|(ms, rs)| (ms.parse().unwrap(), rs.parse().unwrap()))
                                .unwrap(),
                        )
                    })
                    .unwrap();
                Conversion {
                    source_start,
                    destination_start,
                    length,
                }
            })
            .collect();
        entries.sort_unstable();

        let mut table = Self(Default::default());
        for Conversion {
            source_start,
            destination_start,
            length,
        } in entries
        {
            table.0.insert(source_start, destination_start);
            table.0.insert(source_start + length, source_start + length);
        }
        table
    }

    fn convert(&self, ranges: Vec<(usize, bool)>) -> Vec<(usize, bool)> {
        let mut l = ranges.into_iter().peekable();
        let mut r = self.0.iter().peekable();
        let mut stash = Vec::new();
        let mut active_mapping = (0, 0);
        eprintln!("active_mapping = {:?}", &active_mapping);
        let mut active_range_start = None;
        loop {
            match (l.peek(), r.peek()) {
                (Some((l_i, l_v)), r) if r.map_or(true, |(r_i, _)| l_i <= r_i) => {
                    eprintln!("l = ({l_i:?}, {l_v:?})");
                    if !*l_v {
                        assert!(active_range_start.replace(*l_i).is_none());
                    } else {
                        let start = active_range_start.take().unwrap();
                        stash.push((
                            start - active_mapping.0 + active_mapping.1,
                            *l_i - active_mapping.0 + active_mapping.1,
                        ));
                        eprintln!("push {:?}", stash.last().unwrap());
                    }
                    l.next().unwrap();
                }
                (Some(_), None) => unreachable!("this should've been handled above"),
                (Some(_l), Some((i, _))) => {
                    if let Some(start) = active_range_start {
                        stash.push((
                            start - active_mapping.0 + active_mapping.1,
                            *i - active_mapping.0 + active_mapping.1,
                        ));
                        eprintln!("push {:?}", stash.last().unwrap());
                    }
                    active_mapping = r.next().map(|(&a, &b)| (a, b)).unwrap();
                    eprintln!("active_mapping = {:?}", &active_mapping);
                    if active_range_start.is_some() {
                        active_range_start = Some(active_mapping.0);
                    }
                }
                (None, _) => break,
            }
        }
        stash.sort_unstable();
        let mut result = Vec::new();
        for range in stash {
            if range != (0, 0) {
                result.push((range.0, false));
                result.push((range.1, true));
            }
        }
        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Type {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}
impl Type {
    fn next_type(&self) -> Option<Type> {
        use Type::*;
        Some(match self {
            Seed => Soil,
            Soil => Fertilizer,
            Fertilizer => Water,
            Water => Light,
            Light => Temperature,
            Temperature => Humidity,
            Humidity => Location,
            Location => return None,
        })
    }
}
impl FromStr for Type {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Type::*;
        Ok(match s {
            "seed" => Seed,
            "soil" => Soil,
            "fertilizer" => Fertilizer,
            "water" => Water,
            "light" => Light,
            "temperature" => Temperature,
            "humidity" => Humidity,
            "location" => Location,
            _ => unimplemented!("parsing {s:?}"),
        })
    }
}
type Item = (Type, usize);

pub struct Day5;
impl Day5 {
    fn parse_table<'a>(&self, input: impl Iterator<Item = &'a str>) -> ConversionTable {
        let mut table = ConversionTable::default();
        for map in input {
            let mut section = map.lines();
            let source_type: Type = section
                .next()
                .unwrap()
                .split_once('-')
                .unwrap()
                .0
                .parse()
                .unwrap();
            let subtable = table.entry(source_type).or_default();
            for line in section {
                let (destination_start, (source_start, length)) = line
                    .split_once(' ')
                    .map(|(ls, rs)| {
                        (
                            ls.parse().unwrap(),
                            rs.split_once(' ')
                                .map(|(ms, rs)| (ms.parse().unwrap(), rs.parse().unwrap()))
                                .unwrap(),
                        )
                    })
                    .unwrap();
                subtable.insert(Conversion {
                    source_start,
                    destination_start,
                    length,
                });
            }
        }
        table
    }
}
impl TaskA for Day5 {
    fn solve_a(&self) -> Result<String> {
        let mut input = self.input().split("\n\n");
        let seeds: Vec<Item> = input
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|s| (Type::Seed, s.parse().unwrap()))
            .collect();
        let table = self.parse_table(input);
        Ok(seeds
            .into_iter()
            .map(|mut item| {
                while item.0 != Type::Location {
                    let subtable = table.get(&item.0).unwrap();
                    item.0 = item.0.next_type().unwrap();
                    let bound = Conversion {
                        source_start: item.1,
                        destination_start: 0,
                        length: 1,
                    };
                    if let Some(new_item) = subtable
                        .range(bound.clone()..)
                        .next()
                        .and_then(|conv| conv.try_convert(item.1))
                    {
                        item.1 = new_item;
                    } else if let Some(new_item) = subtable
                        .range(..bound)
                        .last()
                        .and_then(|conv| conv.try_convert(item.1))
                    {
                        item.1 = new_item;
                    } else {
                        // item.1 = item.1;
                    }
                }
                item.1
            })
            .min()
            .unwrap()
            .to_string())
    }
}
impl TaskB for Day5 {
    fn solve_b(&self) -> Result<String> {
        let mut input = self.input().split("\n\n");
        let seed_numbers: Vec<usize> = input
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let mut ranges: Vec<(usize, bool)> = seed_numbers
            .chunks(2)
            .flat_map(|chunk| [(chunk[0], false), (chunk[0] + chunk[1], true)])
            .collect();
        // let mut ranges: Vec<(usize, bool)> = seed_numbers
        //     .into_iter()
        //     .flat_map(|i| [(i, false), (i + 1, true)])
        //     .collect();
        ranges.sort_unstable();
        eprintln!("ranges = {:?}", &ranges);

        for section in input {
            let (header, section) = section.split_once('\n').unwrap();
            eprintln!("{header}");
            let table = IntervalConversionTable::parse(section.lines());
            ranges = table.convert(ranges);
            eprintln!("ranges = {:?}", &ranges);
        }
        Ok(ranges.into_iter().map(|r| r.0).min().unwrap().to_string())
    }
}
