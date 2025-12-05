use divan::{Bencher, bench};

fn main() {
    divan::main()
}

#[bench]
fn parse(bencher: Bencher) {
    let input = std::fs::read_to_string("input").expect("Failed to read file");

    bencher.bench(|| {
        let _ = day5::Database::from_string(&input).expect("Failed to parse");
    })
}

#[bench]
fn part1(bencher: Bencher) {
    let input = std::fs::read_to_string("input").expect("Failed to read file");

    bencher.bench(|| {
        day5::part1(&input).expect("Failed part 1");
    })
}

#[bench]
fn part2(bencher: Bencher) {
    let input = std::fs::read_to_string("input").expect("Failed to read file");

    bencher.bench(|| {
        day5::part2(&input).expect("Failed part 1");
    })
}
