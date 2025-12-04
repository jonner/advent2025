use std::{
    collections::{HashSet, VecDeque},
    time::{Duration, SystemTime},
};

use itertools::Itertools;
use tracing::instrument;

fn time<T, F: FnOnce() -> T>(f: F) -> (T, Duration) {
    let start = SystemTime::now();
    let res = f();
    (res, start.elapsed().expect("Failed to get elapsed time"))
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let input = std::fs::read_to_string("input")?;
    let (map, elapsed) = time(|| Map::parse(&input));
    let map = map?;
    println!("Parse time {elapsed:?}");
    let (nlocs, elapsed) = time(|| map.find_accessible_locations().len());
    println!("Part 1: {nlocs} (time: {elapsed:?})");
    let (nlocs, elapsed) = time(move || part2(map));
    println!("Part 2: {nlocs} (time: {elapsed:?})");
    Ok(())
}

fn part2(mut map: Map) -> usize {
    // Initialize queue with all initially accessible locations
    let mut queue: VecDeque<_> = map.find_accessible_locations().into_iter().collect();
    let mut removed = HashSet::new();

    while let Some(pos) = queue.pop_front() {
        // Skip if already removed
        if removed.contains(&pos) {
            continue;
        }

        // Remove this location
        removed.insert(pos);
        map.locations.remove(&pos);

        // Check all neighbors - they might have just become accessible
        for neighbor in map.adjacent_positions(pos) {
            if !removed.contains(&neighbor)
                && map.locations.contains(&neighbor)
                && map.is_accessible(neighbor)
            {
                queue.push_back(neighbor);
            }
        }
    }

    removed.len()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    locations: HashSet<Point>,
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
                    .filter_map(move |(x, ch)| (ch == '@').then_some(Point { x, y }))
            })
            .collect();
        Ok(Self {
            width,
            height,
            locations,
        })
    }

    fn is_accessible(&self, point: Point) -> bool {
        self.adjacent_positions(point)
            .iter()
            .filter(|p| self.locations.contains(p))
            .count()
            < 4
    }

    #[instrument(ret, level = "debug", skip(self))]
    fn find_accessible_locations(&self) -> HashSet<Point> {
        let mut accessible = HashSet::default();
        for point in self.locations.iter() {
            if self.is_accessible(*point) {
                accessible.insert(*point);
            }
        }
        accessible
    }

    #[instrument(ret, level = "trace", skip(self))]
    fn adjacent_positions(&self, point: Point) -> Vec<Point> {
        let x_ranges = point.x.saturating_sub(1)..=self.width.min(point.x + 1);
        let y_ranges = point.y.saturating_sub(1)..=self.height.min(point.y + 1);
        x_ranges
            .cartesian_product(y_ranges)
            .filter_map(|(x, y)| {
                if x != point.x || y != point.y {
                    Some(Point { x, y })
                } else {
                    None
                }
            })
            .collect()
    }

    fn _print_accessible_locations(&self) -> String {
        let mut output = String::with_capacity((self.width + 1) * self.height);
        let locs = self.find_accessible_locations();
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point { x, y };
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
        println!("{}", map._print_accessible_locations());
    }
}
