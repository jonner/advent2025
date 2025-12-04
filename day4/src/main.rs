use std::{
    collections::HashSet,
    time::{Duration, SystemTime},
};

use itertools::Itertools;
use tracing::instrument;

fn time<T, F: Fn() -> T>(f: F) -> (T, Duration) {
    let start = SystemTime::now();
    let res = f();
    (res, start.elapsed().expect("Failed to get elapsed time"))
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?;
    let map = Map::parse(&input)?;
    let (output, elapsed) = time(|| map.print_accessible_locations());
    println!("Part 1:\n{output}\ntime: {elapsed:?}");
    let (nlocs, elapsed) = time(|| map.find_accessible_locations().len());
    println!("Part 1: {nlocs} (time: {elapsed:?})");
    Ok(())
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    locations: HashSet<(usize, usize)>,
}

impl Map {
    #[instrument(ret, level = "debug")]
    fn parse(input: &str) -> anyhow::Result<Self> {
        let width = input.lines().count();
        let height = input.lines().next().map_or(0, |line| line.chars().count());
        let locations = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, ch)| match ch {
                        '@' => Some((x, y)),
                        _ => None,
                    })
            })
            .collect();
        Ok(Self {
            width,
            height,
            locations,
        })
    }

    #[instrument(ret, level = "debug", skip(self))]
    fn find_accessible_locations(&self) -> Vec<(usize, usize)> {
        let mut accessible = Vec::default();
        for (x, y) in self.locations.iter() {
            if self
                .adjacent_positions(*x, *y)
                .iter()
                .filter(|p| self.locations.contains(p))
                .count()
                < 4
            {
                accessible.push((*x, *y));
            }
        }
        accessible
    }

    #[instrument(ret, level = "trace", skip(self))]
    fn adjacent_positions(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let x_ranges = x.saturating_sub(1)..=self.width.min(x + 1);
        let y_ranges = y.saturating_sub(1)..=self.height.min(y + 1);
        x_ranges
            .cartesian_product(y_ranges)
            .filter(|point| point.0 != x || point.1 != y)
            .collect()
    }

    fn print_accessible_locations(&self) -> String {
        let mut output = String::with_capacity((self.width + 1) * self.height);
        let locs = self.find_accessible_locations();
        for y in 0..self.height {
            for x in 0..self.width {
                let point = (x, y);
                if locs.contains(&point) {
                    output.push('x');
                } else if self.locations.contains(&point) {
                    output.push('@');
                } else {
                    output.push('.');
                }
                if x % self.width == self.width - 1 {
                    output.push('\n');
                }
            }
        }
        output.push('\n');
        output.push_str(&format!("Total Accessible Locations: {}", locs.len()));
        output
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_log::test;

    const EXAMPLE_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_parse() {
        let map = Map::parse(EXAMPLE_INPUT).expect("Failed to parse input");
        assert_eq!(10, map.width);
        assert_eq!(10, map.height);
        assert_eq!(71, map.locations.len());
    }

    #[test]
    fn test_example_part1() {
        let map = Map::parse(EXAMPLE_INPUT).expect("Failed to parse input");
        let locs = map.find_accessible_locations();
        assert_eq!(13, locs.len());
        println!("{}", map.print_accessible_locations());
    }
}
