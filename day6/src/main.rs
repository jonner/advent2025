fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let input = std::fs::read_to_string("input")?;
    println!("Part 1: {}", day6::part1(&input)?);
    Ok(())
}
