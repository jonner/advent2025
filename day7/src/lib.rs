use std::{collections::HashSet, ops::Add};

use anyhow::anyhow;
use tracing::{instrument, trace};

pub fn part1(input: &str) -> anyhow::Result<String> {
    let mut manifold = Manifold::parse(input)?;
    let splits = manifold.run();
    Ok(splits.to_string())
}

pub fn part2(input: &str) -> anyhow::Result<String> {
    let manifold = Manifold::parse(input)?;
    todo!()
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
    pub fn step_beam(&mut self) -> Option<u32> {
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

    pub fn run(&mut self) -> u32 {
        let mut total_splits = 0;
        while let Some(splits) = self.step_beam() {
            total_splits += splits;
        }
        total_splits
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
}
