use day4::Map;

fn main() {
    divan::main();
}

#[divan::bench]
fn parser(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("input").expect("Failed to read input");
    bencher.bench(|| {
        let _map = Map::parse(&input).expect("failed to parse input");
    })
}

#[divan::bench]
fn part1(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("input").expect("Failed to read input");

    bencher.bench(|| {
        let map = Map::parse(&input).expect("Failed to parse input");
        let _nlocs = map.find_accessible_locations().len();
    })
}

#[divan::bench]
fn part2(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("input").expect("Failed to read input");

    bencher.bench(|| {
        let mut map = Map::parse(&input).expect("Failed to parse input");
        let _nlocs = map.part2();
    })
}

#[divan::bench(sample_count = 20)]
fn part2_iterate(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("input").expect("Failed to read input");

    bencher.bench(|| {
        let mut map = Map::parse(&input).expect("Failed to parse input");
        let _nlocs = map.part2_iterate();
    })
}
