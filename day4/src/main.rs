use day4::*;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let input = std::fs::read_to_string("input")?;
    let mut map = Map::parse(&input)?;
    let nlocs = map.find_accessible_locations().len();
    println!("Part 1: {nlocs}");
    let nlocs = map.part2();
    println!("Part 2: {nlocs}");
    Ok(())
}
