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
        let db = day5::Database::from_string(&input).expect("Failed to parse");
        let _ = db.fresh_ingredients();
    })
}
