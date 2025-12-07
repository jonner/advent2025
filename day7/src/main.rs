use day7::part1;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?;
    println!("Part1: {}", part1(&input)?);
    Ok(())
}
