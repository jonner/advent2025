use tracing::{debug, info, instrument, trace};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let input = std::fs::read_to_string("input")?;
    let ranges = parse(&input)?;
    let sum: i64 = ranges
        .into_iter()
        .filter_map(|range| range.find_invalid_ids())
        .flatten()
        .sum();
    println!("Part 1: {sum}");
    Ok(())
}

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

fn is_odd(val: u32) -> bool {
    val.rem_euclid(2) != 0
}

impl Range {
    #[instrument(ret, level = "trace")]
    pub fn find_invalid_ids(&self) -> Option<Vec<i64>> {
        let mut invalid_ids = Vec::default();
        let mut start_digits = self
            .start
            .checked_ilog10()
            .expect("Couldn't calculate digits of range start")
            + 1;
        let mut end_digits = self
            .end
            .checked_ilog10()
            .expect("Couldn't calculate digits of range end")
            + 1;
        if is_odd(start_digits) && start_digits == end_digits {
            debug!(
                ?self,
                "range only includes numbers with an odd number of digits. ID can't be two repeating numbers."
            );
            return None;
        }
        let mut start = self.start;
        let mut end = self.end;
        if is_odd(start_digits) {
            debug!(start, "start has an odd number of digits. adjusting");
            // advance to the next digit
            start = 10_i64.pow(start_digits);
            start_digits += 1;
        };
        if is_odd(end_digits) {
            debug!(end, "end has an odd number of digits. adjusting");
            // advance to the next digit
            end_digits -= 1;
            end = 10_i64.pow(end_digits) - 1;
        };
        assert!(end_digits >= start_digits);
        debug!(start, end, "starting to search");
        let half_digits = start_digits / 2;
        let last_half_start = start % 10_i64.pow(half_digits);
        let last_half_end = end % 10_i64.pow(half_digits);
        let first_half_start = (start - last_half_start) / 10_i64.pow(start_digits / 2);
        let first_half_end = (end - last_half_end) / 10_i64.pow(end_digits / 2);
        debug!(
            half_digits,
            last_half_start, first_half_start, last_half_end, first_half_end,
        );

        let range_end = last_half_end.max(first_half_end);
        if range_end < first_half_start {
            debug!(
                first_half_start,
                range_end,
                "last half of range end is smaller than first half of range start. no possibilities found"
            );
            return None;
        }
        debug!("Checking doubled numbers between {first_half_start} and {range_end}");
        for i in first_half_start..=range_end {
            let id = i * 10_i64.pow(half_digits) + i;
            trace!(id, "Checking repeated number");
            if id > self.end {
                debug!(id, "already exceeded range early. aborting");
                break;
            }
            if self.in_range(id) {
                info!(id, ?self, "Found invalid id");
                invalid_ids.push(id);
            }
        }
        // check if first half of the start number repeated twice is smaller than the end of the range
        // todo!()
        Some(invalid_ids)
    }

    fn in_range(&self, val: i64) -> bool {
        val >= self.start && val <= self.end
    }
}

#[instrument(ret, level = "trace")]
fn parse(input: &str) -> anyhow::Result<Vec<Range>> {
    let input = input.trim();
    let mut ranges: Vec<Range> = Vec::default();
    for item in input.split(',') {
        let numbers = item
            .split_once('-')
            .ok_or_else(|| anyhow::anyhow!("Failed to find a range separator '-'"))?;
        let range = Range {
            start: numbers.0.parse()?,
            end: numbers.1.parse()?,
        };
        ranges.push(range)
    }
    Ok(ranges)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test_log::test]
    fn test_parse() {
        let ranges = parse(EXAMPLE_INPUT).expect("Failed to parse input");
        assert_eq!(11, ranges.len());

        assert_eq!(11, ranges[0].start);
        assert_eq!(22, ranges[0].end);

        assert_eq!(2121212118, ranges[10].start);
        assert_eq!(2121212124, ranges[10].end);
    }

    #[test_log::test]
    fn test_examples() {
        let ranges = parse(EXAMPLE_INPUT).expect("Failed to parse input");
        let mut invalid_ids = Vec::default();
        for range in ranges {
            if let Some(ids) = range.find_invalid_ids() {
                invalid_ids.extend(ids.into_iter());
            }
        }

        assert_eq!(8, invalid_ids.len());
        assert_eq!(1227775554_i64, invalid_ids.into_iter().sum());
    }
}
