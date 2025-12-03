use anyhow::anyhow;
use tracing::{debug, instrument, warn};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let input = std::fs::read_to_string("input")?;
    let banks = parse(&input)?;
    let sum = part1(banks);
    println!("Part 1: {sum}");
    Ok(())
}

fn part1(banks: Vec<BatteryBank>) -> u32 {
    banks.into_iter().map(|bank| bank.max_joltage()).sum()
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
        let (first_digit, positions) = first_elements.iter().enumerate().fold(
            (0, Vec::default()),
            |acc: (u32, Vec<usize>), (i, item)| {
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
        let starting_pos = positions.first().expect("positions was empty") + 1;
        let remaining = &self.cells[starting_pos..];
        let second_digit = remaining
            .iter()
            .max()
            .expect("Unable to find max of remaining digits");
        first_digit * 10 + second_digit
    }
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
    fn test_example() {
        let batteries = parse(TEST_INPUT).expect("Failed to parse input");
        assert_eq!(98, batteries[0].max_joltage());
        assert_eq!(89, batteries[1].max_joltage());
        assert_eq!(78, batteries[2].max_joltage());
        assert_eq!(92, batteries[3].max_joltage());
    }

    #[test]
    fn test_parse() {
        let batteries = parse(TEST_INPUT).expect("Failed to parse input");
        assert_eq!(4, batteries.len());
        assert_eq!(9, batteries[0].cells[0]);
        assert_eq!(2, batteries[0].cells[7]);
    }
}
