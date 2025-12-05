fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let input = std::fs::read_to_string("input")?;
    let res = day5::part1(&input)?;
    println!("Part 1: {res}");
    let res = day5::part2(&input)?;
    println!("Part 2: {res}");
    Ok(())
}
