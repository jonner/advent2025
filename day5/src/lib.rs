use anyhow::anyhow;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{count, separated_list1},
    sequence::separated_pair,
};

pub fn part1() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?;
    let database = Database::from_string(&input)?;
    println!("Part 1: {}", database.fresh_ingredients().len());
    Ok(())
}

#[derive(Debug, PartialEq)]
pub(crate) struct Range {
    pub(crate) lower: u64,
    pub(crate) upper: u64,
}

impl Range {
    pub fn contains(&self, id: u64) -> bool {
        id >= self.lower && id <= self.upper
    }
}

#[derive(Debug)]
pub struct Database {
    pub(crate) fresh: Vec<Range>,
    pub(crate) ingredients: Vec<u64>,
}

impl Database {
    pub fn from_string(input: &str) -> anyhow::Result<Self> {
        parse(input)
    }

    pub fn fresh_ingredients(&self) -> Vec<u64> {
        self.ingredients
            .iter()
            .filter(|&item| self.is_fresh(*item))
            .copied()
            .collect()
    }
    pub(crate) fn is_fresh(&self, id: u64) -> bool {
        self.fresh.iter().any(|f| f.contains(id))
    }
}

pub(crate) fn parse_range(input: &str) -> IResult<&str, Range> {
    separated_pair(complete::u64, tag("-"), complete::u64)
        .map(|(lower, upper)| Range { lower, upper })
        .parse(input)
}

pub(crate) fn parse(input: &str) -> anyhow::Result<Database> {
    let (_, database) = separated_pair(
        separated_list1(newline, parse_range),
        count(newline, 2),
        separated_list1(newline, complete::u64),
    )
    .map(|(ranges, ids)| Database {
        fresh: ranges,
        ingredients: ids,
    })
    .parse(input)
    .map_err(|e| anyhow!("Failed to parse: {e}"))?;
    Ok(database)
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use test_log::test;

    const EXAMPLE_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn test_parse() {
        let database = parse(EXAMPLE_INPUT).expect("Failed to parse");
        assert_eq!(6, database.ingredients.len());
        assert!(database.is_fresh(3));
        assert!(database.is_fresh(4));
        assert!(database.is_fresh(5));
        assert!(!database.is_fresh(6));
        assert!(database.is_fresh(10));
        assert!(database.is_fresh(11));
        assert!(database.is_fresh(12));
        assert!(database.is_fresh(13));
        assert!(database.is_fresh(14));
        assert!(database.is_fresh(15));
        assert!(database.is_fresh(16));
        assert!(database.is_fresh(17));
        assert!(database.is_fresh(18));
        assert!(database.is_fresh(19));
        assert!(database.is_fresh(20));
        assert!(!database.is_fresh(21));
    }

    #[test]
    fn test_part1() {
        let database = parse(EXAMPLE_INPUT).expect("Failed to parse");
        let fresh = database.fresh_ingredients();
        assert_eq!(3, fresh.len());
        assert!(fresh.contains(&5));
        assert!(fresh.contains(&11));
        assert!(fresh.contains(&17));
        assert!(!fresh.contains(&1));
        assert!(!fresh.contains(&8));
        assert!(!fresh.contains(&32));
    }
}
