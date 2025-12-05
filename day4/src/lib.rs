use std::collections::HashSet;

use std::collections::VecDeque;

use itertools::Itertools;
use tracing::instrument;
use tracing::trace;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub locations: HashSet<Point>,
}

impl Map {
    #[instrument(ret, level = "debug")]
    pub fn parse(input: &str) -> anyhow::Result<Self> {
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

    pub fn part2_iterate(&mut self) -> usize {
        let mut total = 0;
        loop {
            let locs = self.find_accessible_locations();
            if locs.is_empty() {
                break;
            }
            total += locs.len();
            trace!(?locs, "Removing accessible locations");
            self.locations = &self.locations - &locs;
        }
        total
    }

    pub fn part2(&mut self) -> usize {
        // Initialize queue with all initially accessible locations
        let mut queue: VecDeque<_> = self.find_accessible_locations().into_iter().collect();
        let mut removed = HashSet::new();

        while let Some(pos) = queue.pop_front() {
            // Skip if already removed
            if removed.contains(&pos) {
                continue;
            }

            // Remove this location
            removed.insert(pos);
            self.locations.remove(&pos);

            // Check all neighbors - they might have just become accessible
            for neighbor in self.adjacent_positions(pos) {
                if !removed.contains(&neighbor)
                    && self.locations.contains(&neighbor)
                    && self.is_accessible(neighbor)
                {
                    queue.push_back(neighbor);
                }
            }
        }

        removed.len()
    }

    fn is_accessible(&self, point: Point) -> bool {
        self.adjacent_positions(point)
            .iter()
            .filter(|p| self.locations.contains(p))
            .count()
            < 4
    }

    #[instrument(ret, level = "debug", skip(self))]
    pub fn find_accessible_locations(&self) -> HashSet<Point> {
        let mut accessible = HashSet::default();
        for point in self.locations.iter() {
            if self.is_accessible(*point) {
                accessible.insert(*point);
            }
        }
        accessible
    }

    #[instrument(ret, level = "trace", skip(self))]
    pub fn adjacent_positions(&self, point: Point) -> Vec<Point> {
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

    pub fn _print_accessible_locations(&self) -> String {
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
