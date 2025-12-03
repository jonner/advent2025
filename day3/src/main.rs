use std::time::SystemTime;

use anyhow::anyhow;
use tracing::{debug, instrument, warn};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let input = std::fs::read_to_string("input")?;
    let banks = parse(&input)?;
    let start = SystemTime::now();
    let sum = part1(&banks);
    println!("Part 1: {sum} (time: {:?})", start.elapsed()?);
    let start = SystemTime::now();
    let sum = part2(&banks);
    println!("Part 2: {sum} (time: {:?})", start.elapsed()?);
    Ok(())
}

fn part1(banks: &[BatteryBank]) -> u32 {
    banks.iter().map(|bank| bank.max_joltage()).sum()
}

fn part2(banks: &[BatteryBank]) -> u64 {
    banks.iter().map(|bank| bank.max_joltage_2()).sum()
}

#[derive(Debug)]
struct BatteryBank {
    cells: Vec<u32>,
}

impl BatteryBank {
    #[instrument(ret, level = "debug")]
    fn max_joltage(&self) -> u32 {
        let Some((_last_element, first_elements)) = self.cells.split_last() else {
            warn!("Unable to split last");
            return 0;
        };
        let (first_digit, first_position) =
            first_elements
                .iter()
                .enumerate()
                .fold((0, 0), |acc: (u32, usize), (i, item)| {
                    let mut max = acc.0;
                    let mut position = acc.1;
                    if *item > max {
                        max = *item;
                        position = i;
                    }
                    (max, position)
                });
        debug!(?first_digit, ?first_position);
        let starting_pos = first_position + 1;
        let remaining = &self.cells[starting_pos..];
        let second_digit = remaining
            .iter()
            .max()
            .expect("Unable to find max of remaining digits");
        first_digit * 10 + second_digit
    }

    fn max_joltage_2(&self) -> u64 {
        let mut total: u64 = 0;
        let mut pos = 0;
        for n in (0..12).rev() {
            debug!("finding max digit from {pos} to -{n}");
            let (digit, found_position) = find_max_ignoring_end_n(&self.cells[pos..], n);
            total += 10_u64.pow(n as u32) * digit as u64;
            pos += found_position + 1;
        }
        total
    }
}

#[instrument(ret, level = "debug")]
fn find_max_ignoring_end_n(cells: &[u32], end_n: usize) -> (u32, usize) {
    let (first_elements, _last_elements) = cells.split_at(cells.len() - end_n);
    debug!(?first_elements);
    let (first_digit, positions) = first_elements.iter().enumerate().fold(
        (0, Vec::default()),
        |acc: (u32, Vec<usize>), (i, item)| {
            debug!(?acc, i, item);
            let mut max = acc.0;
            let mut positions = acc.1;
            if *item > max {
                max = *item;
                positions.clear();
                positions.push(i);
            } else if *item == max {
                positions.push(i);
            }
            (max, positions)
        },
    );
    debug!(?first_digit, ?positions);
    let pos = positions.first().expect("positions was empty");
    (first_digit, *pos)
}

fn parse(input: &str) -> anyhow::Result<Vec<BatteryBank>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    ch.to_digit(10)
                        .ok_or_else(|| anyhow!("Failed to convert {ch} to a digit"))
                })
                .collect::<Result<_, _>>()
                .map(|v| BatteryBank { cells: v })
        })
        .collect::<Result<Vec<_>, _>>()
}

#[cfg(test)]
mod test {
    use super::*;
    use test_log::test;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        let batteries = parse(TEST_INPUT).expect("Failed to parse input");
        assert_eq!(98, batteries[0].max_joltage());
        assert_eq!(89, batteries[1].max_joltage());
        assert_eq!(78, batteries[2].max_joltage());
        assert_eq!(92, batteries[3].max_joltage());
    }

    #[test]
    fn test_part2() {
        let batteries = parse(TEST_INPUT).expect("Failed to parse input");
        assert_eq!(987654321111, batteries[0].max_joltage_2());
        assert_eq!(811111111119, batteries[1].max_joltage_2());
        assert_eq!(434234234278, batteries[2].max_joltage_2());
        assert_eq!(888911112111, batteries[3].max_joltage_2());
    }

    #[test]
    fn test_parse() {
        let batteries = parse(TEST_INPUT).expect("Failed to parse input");
        assert_eq!(4, batteries.len());
        assert_eq!(9, batteries[0].cells[0]);
        assert_eq!(2, batteries[0].cells[7]);
    }
}
