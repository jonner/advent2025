use itertools::Itertools;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{self, char, line_ending, space0, space1},
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair},
};
use std::iter::Iterator;
use tracing::{debug, instrument};

pub fn part1(input: &str) -> anyhow::Result<String> {
    let (_, problems) = parse(input).expect("parsing failed");
    let sum: i64 = problems.into_iter().map(|problem| problem.compute()).sum();
    Ok(sum.to_string())
}

pub fn part2(input: &str) -> anyhow::Result<String> {
    let problems = parse2(input);
    let sum: i64 = problems.into_iter().map(|problem| problem.compute()).sum();
    Ok(sum.to_string())
}

#[derive(Debug)]
pub enum Operation {
    Add,
    Multiply,
}

// Source - https://stackoverflow.com/a/55292215
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
pub fn parse2(input: &str) -> Vec<Problem> {
    let chariters = input.lines().map(|l| l.chars()).collect::<Vec<_>>();
    // zip the same column of each line into a vector. Essentially transposing
    // between lines and columns
    let cols = Multizip(chariters).collect::<Vec<_>>();
    debug!(?cols);
    let problems = cols
        .iter()
        // process the line from the back to the front
        .rev()
        .map(|col| {
            debug!(?col);
            (
                // we know the operator is in the last row, so construct a
                // string of the characters in the first lines...
                col.iter()
                    .take(col.len() - 1)
                    .collect::<String>()
                    .trim()
                    .parse::<i64>(),
                // ...and convert the last line into an Operator (if there is one)
                col.iter().last().and_then(|c| match c {
                    '*' => Some(Operation::Multiply),
                    '+' => Some(Operation::Add),
                    ' ' => None,
                    _ => panic!("Unexpected operation"),
                }),
            )
        })
        // Process numbers until we get totally a blank column (indicated by a
        // ParseIntError), and construct a Problem object from them
        .batching(|it| {
            let mut problemop: Option<Operation> = None;
            let mut args: Vec<i64> = Vec::default();
            loop {
                let Some((iparseresult, op)) = it.next() else {
                    break;
                };
                debug!(?iparseresult, ?op);
                if let Some(oper) = op {
                    problemop = Some(oper);
                }
                match iparseresult {
                    Ok(n) => args.push(n),
                    Err(_e) => break,
                }
            }
            Some(args)
                .zip(problemop)
                .map(|(args, op)| Problem { args, op })
        })
        .collect::<Vec<_>>();
    debug!(?problems);
    problems
}

#[instrument(ret, level = "trace")]
pub fn parse(input: &str) -> IResult<&str, Vec<Problem>> {
    separated_pair(
        separated_list1(
            line_ending::<&str, nom::error::Error<&str>>,
            delimited(space0, separated_list1(space1, complete::i64), space0),
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
        let (_, problems) = parse(EXAMPLE_INPUT).expect("parsing failed");
        assert_eq!(4, problems.len());
        assert_eq!(33210, problems[0].compute());
        assert_eq!(490, problems[1].compute());
        assert_eq!(4243455, problems[2].compute());
        assert_eq!(401, problems[3].compute());
    }

    #[test]
    fn test_parse2() {
        let problems = parse2(EXAMPLE_INPUT);
        assert_eq!(4, problems.len());
        assert_eq!(1058, problems[0].compute());
        assert_eq!(3253600, problems[1].compute());
        assert_eq!(625, problems[2].compute());
        assert_eq!(8544, problems[3].compute());
    }
}
