use day7::Manifold;

fn main() {
    divan::main()
}

#[divan::bench]
fn parse(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("input").expect("Failed to read file");
    bencher.bench(|| Manifold::parse(&input).expect("failed to parse"))
}

#[divan::bench]
fn part1(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("input").expect("Failed to read file");
    bencher.bench(|| {
        let mut manifold = Manifold::parse(&input).expect("failed to parse");
        let _ = manifold.run();
    })
}

#[divan::bench]
fn part2(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("input").expect("Failed to read file");
    bencher.bench(|| {
        let mut manifold = Manifold::parse(&input).expect("failed to parse");
        let _ = manifold.timelines();
    })
}
