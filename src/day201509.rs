use std::collections::HashMap;

use crate::prelude::*;

pub struct Day201509;
impl Day201509 {
    fn parsed_input(&self) -> HashMap<String, HashMap<String, usize>> {
        let mut map: HashMap<String, HashMap<String, usize>> = HashMap::new();
        for line in self.input().lines() {
            let (route, distance) = line.split_once(" = ").unwrap();
            let distance: usize = distance.parse().unwrap();
            let (a, b) = route.split_once(" to ").unwrap();
            map.entry(a.to_string())
                .or_default()
                .insert(b.to_string(), distance);
            map.entry(b.to_string())
                .or_default()
                .insert(a.to_string(), distance);
        }
        map
    }
    fn routes(&self, max: bool) -> usize {
        let map = self.parsed_input();
        let cities: Vec<_> = map.keys().collect();
        let routes = cities.iter().permutations(cities.len()).map(|route| {
            route
                .into_iter()
                .tuple_windows()
                .map(|(&a, &b)| map[a][b])
                .sum::<usize>()
        });
        if max { routes.max() } else { routes.min() }.unwrap()
    }
}
impl TaskA for Day201509 {
    fn solve_a(&self) -> Result<String> {
        Ok(self.routes(false).to_string())
    }
}
impl TaskB for Day201509 {
    fn solve_b(&self) -> Result<String> {
        Ok(self.routes(true).to_string())
    }
}
