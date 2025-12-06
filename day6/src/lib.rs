use nom::{
    Parser,
    branch::alt,
    character::complete::{self, char, line_ending, space0, space1},
    multi::separated_list1,
    sequence::{pair, separated_pair},
};
use std::iter::Iterator;
use tracing::instrument;

pub fn part1(input: &str) -> anyhow::Result<String> {
    let problems = parse(input);
    let sum: i64 = problems.into_iter().map(|problem| problem.compute()).sum();
    Ok(sum.to_string())
}

#[derive(Debug)]
pub enum Operation {
    Add,
    Multiply,
}

// Source - https://stackoverflow.com/a
// Posted by Shepmaster
// Retrieved 2025-12-06, License - CC BY-SA 4.0
struct Multizip<T>(Vec<T>);

impl<T> Iterator for Multizip<T>
where
    T: Iterator,
{
    type Item = Vec<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter_mut().map(Iterator::next).collect()
    }
}

#[derive(Debug)]
pub struct Problem {
    args: Vec<i64>,
    op: Operation,
}

impl Problem {
    #[instrument(ret, level = "trace")]
    pub fn compute(&self) -> i64 {
        match self.op {
            Operation::Add => self.args.iter().sum(),
            Operation::Multiply => self.args.iter().product(),
        }
    }
}

#[instrument(ret, level = "trace")]
pub fn parse(input: &str) -> Vec<Problem> {
    let (_, problems) = separated_pair(
        separated_list1(
            pair(space0, line_ending::<&str, nom::error::Error<&str>>),
            separated_list1(space1, complete::i64),
        )
        .map(|vv| {
            let vec_of_iters = vv.into_iter().map(|v| v.into_iter()).collect();
            Multizip(vec_of_iters).collect::<Vec<_>>()
        }),
        pair(space0, line_ending),
        separated_list1(
            space1,
            alt((
                char('*').map(|_| Operation::Multiply),
                char('+').map(|_| Operation::Add),
            )),
        ),
    )
    .map(|(args, ops)| {
        args.into_iter()
            .zip(ops.into_iter())
            .map(|(args, op)| Problem { args, op })
            .collect::<Vec<_>>()
    })
    .parse(input)
    .expect("Failed to parse");
    problems
}

#[cfg(test)]
mod test {
    use super::*;
    use test_log::test;

    const EXAMPLE_INPUT: &str = "123 328  51 64 
45 64  387 23 
6 98  215 314
*   +   *   +  
";

    #[test]
    fn test_parse() {
        let problems = parse(EXAMPLE_INPUT);
        assert_eq!(4, problems.len());
        assert_eq!(33210, problems[0].compute());
        assert_eq!(490, problems[1].compute());
        assert_eq!(4243455, problems[2].compute());
        assert_eq!(401, problems[3].compute());
    }
}
