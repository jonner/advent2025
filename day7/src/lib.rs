use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use anyhow::anyhow;
use tracing::{instrument, trace};

pub fn part1(input: &str) -> anyhow::Result<String> {
    let mut manifold = Manifold::parse(input)?;
    let splits = manifold.run();
    Ok(splits.to_string())
}

pub fn part2(input: &str) -> anyhow::Result<String> {
    let mut manifold = Manifold::parse(input)?;
    let timelines = manifold.timelines();
    Ok(timelines.to_string())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    #[instrument(ret, skip(self), level = "trace")]
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

const DOWN: Point = Point { x: 0, y: 1 };
const LEFT: Point = Point { x: -1, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };

#[derive(Debug, PartialEq, Eq)]
pub enum Symbol {
    Start,
    Splitter,
    Empty,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Manifold {
    data: Vec<Vec<char>>,
    start: Point,
    beams: HashSet<Point>,
}

impl Manifold {
    #[instrument(ret, skip(input), level = "debug")]
    pub fn parse(input: &str) -> anyhow::Result<Self> {
        let data = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        trace!(?data);
        let start = (|| {
            for (y, line) in data.iter().enumerate() {
                for (x, ch) in line.iter().enumerate() {
                    if ch == &'S' {
                        return Ok(Point {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                }
            }
            Err(anyhow!("Didn't find start"))
        })()?;
        Ok(Manifold {
            data,
            start,
            beams: Default::default(),
        })
    }

    pub fn width(&self) -> i32 {
        self.data.first().map(|e| e.len()).unwrap_or(0) as i32
    }

    pub fn height(&self) -> i32 {
        self.data.len() as i32
    }

    pub fn at(&self, point: &Point) -> Symbol {
        match self
            .data
            .get(point.y as usize)
            .and_then(|line| line.get(point.x as usize))
        {
            Some('S') => Symbol::Start,
            Some('^') => Symbol::Splitter,
            _ => Symbol::Empty,
        }
    }

    #[instrument(ret, skip(self), level = "trace")]
    pub fn step_beam(&mut self) -> Option<u64> {
        if self.beams.is_empty() {
            trace!("starting beam...");
            self.beams.insert(self.start);
            return Some(0);
        }
        let mut splits = 0;
        let mut beams = HashSet::default();
        std::mem::swap(&mut beams, &mut self.beams);
        for beam in beams {
            trace!(?beam, "processing beam");
            let next_loc = beam + DOWN;
            if next_loc.y >= self.height() {
                return None;
            }
            let sym = self.at(&next_loc);
            trace!(?next_loc, ?sym);
            if sym == Symbol::Splitter {
                splits += 1;
                self.beams.insert(next_loc + LEFT);
                self.beams.insert(next_loc + RIGHT);
                trace!(?self, "split beams")
            } else {
                self.beams.insert(next_loc);
            }
        }
        Some(splits)
    }

    pub fn run(&mut self) -> u64 {
        let mut total_splits = 0;
        while let Some(splits) = self.step_beam() {
            total_splits += splits;
        }
        total_splits
    }

    #[instrument(ret, skip(self))]
    pub fn step_timeline(&self, timelines: Timelines) -> Option<Timelines> {
        let mut new_timelines = Timelines::default();
        for (cur_point, multiples) in timelines.data.iter() {
            let next_point = *cur_point + DOWN;
            if next_point.y > self.height() {
                return None;
            }
            if self.at(&next_point) == Symbol::Splitter {
                new_timelines.insert(next_point + RIGHT, *multiples);
                new_timelines.insert(next_point + LEFT, *multiples);
            } else {
                new_timelines.insert(next_point, *multiples)
            }
        }
        Some(new_timelines)
    }

    #[instrument(ret, skip(self))]
    pub fn timelines(&mut self) -> u64 {
        let mut ntimelines = 1;
        let mut timelines = {
            let mut t = Timelines::default();
            t.insert(self.start, 1);
            t
        };
        while let Some(new_timelines) = self.step_timeline(timelines) {
            ntimelines = new_timelines.data.values().sum();
            timelines = new_timelines;
        }
        ntimelines
    }
}

#[derive(Debug, Default)]
pub struct Timelines {
    pub data: HashMap<Point, u64>,
}

impl Timelines {
    #[instrument]
    pub fn insert(&mut self, point: Point, num: u64) {
        self.data
            .entry(point)
            .and_modify(|val| *val += num)
            .or_insert(num);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_log::test;

    const EXAMPLE_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_parse() {
        let manifold = Manifold::parse(EXAMPLE_INPUT).expect("Failed to parse");
        assert_eq!(15, manifold.width());
        assert_eq!(16, manifold.height());
        assert_eq!(Point { x: 7, y: 0 }, manifold.start);
        assert_eq!(Symbol::Splitter, manifold.at(&Point { x: 1, y: 14 }));
    }

    #[test]
    fn test_part1() {
        let mut manifold = Manifold::parse(EXAMPLE_INPUT).expect("Failed to parse");
        assert_eq!(21, manifold.run());
    }

    #[test]
    fn test_part2() {
        let mut manifold = Manifold::parse(EXAMPLE_INPUT).expect("Failed to parse");
        assert_eq!(40, manifold.timelines());
    }
}
