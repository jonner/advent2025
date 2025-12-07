fn main() {
    divan::main()
}

#[divan::bench(sample_count = 1000)]
fn parse(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("input").expect("Failed to read file");
    bencher.bench(|| _ = day6::parse(&input));
}

#[divan::bench(sample_count = 1000)]
fn parse2(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("input").expect("Failed to read file");
    bencher.bench(|| _ = day6::parse2(&input));
}

#[divan::bench(sample_count = 1000)]
fn part1(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("input").expect("Failed to read file");
    bencher.bench(|| day6::part1(&input).expect("Failed part1"));
}

#[divan::bench(sample_count = 1000)]
fn part2(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("input").expect("Failed to read file");
    bencher.bench(|| day6::part2(&input).expect("Failed part2"));
}
