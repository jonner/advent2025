use std::str::FromStr;

const INITIAL_POSITION: i32 = 50;
const DIAL_RANGE: i32 = 100;

#[tracing::instrument(level = "debug", ret)]
fn apply_rotation(current_pos: i32, rotation: Rotation) -> (u32, i32) {
    let movement = match rotation {
        Rotation::Left(n) => -n,
        Rotation::Right(n) => n,
    };
    let target = current_pos + movement;
    let mut crosses: u32 = (target / DIAL_RANGE).abs().try_into().unwrap();
    let new_pos = target.rem_euclid(DIAL_RANGE);
    if current_pos != 0 && target <= 0 {
        // if we're moving left past 0, add another crossing point
        crosses += 1;
    }
    (crosses, new_pos)
}

#[rstest::rstest]
#[case(50, Rotation::Right(50), (1, 0))]
#[case(50, Rotation::Left(60), (1, 90))]
#[case(1, Rotation::Left(8), (1, 93))]
#[case(50, Rotation::Left(50), (1, 0))]
#[case(50, Rotation::Right(250), (3, 0))]
#[case(82, Rotation::Left(682), (7, 0))]
fn test_apply_rotation(#[case] start: i32, #[case] rotation: Rotation, #[case] res: (u32, i32)) {
    assert_eq!(apply_rotation(start, rotation), res);
}

#[test]
fn test_steps_part2() {
    let mut s = State2(50);
    assert_eq!((1u32, 82), apply_rotation(s.0, Rotation::Left(68)));
    s.apply(Rotation::Left(68));
    assert_eq!((0, 52), apply_rotation(s.0, Rotation::Left(30)));
    s.apply(Rotation::Left(30));
    assert_eq!((1, 0), apply_rotation(s.0, Rotation::Right(48)));
    s.apply(Rotation::Right(48));
    assert_eq!((0, 95), apply_rotation(s.0, Rotation::Left(5)));
    s.apply(Rotation::Left(5));
    assert_eq!((1, 55), apply_rotation(s.0, Rotation::Right(60)));
    s.apply(Rotation::Right(60));
    assert_eq!((1, 0), apply_rotation(s.0, Rotation::Left(55)));
    s.apply(Rotation::Left(55));
    assert_eq!((0, 99), apply_rotation(s.0, Rotation::Left(1)));
    s.apply(Rotation::Left(1));
    assert_eq!((1, 0), apply_rotation(s.0, Rotation::Left(99)));
    s.apply(Rotation::Left(99));
    assert_eq!((0, 14), apply_rotation(s.0, Rotation::Right(14)));
    s.apply(Rotation::Right(14));
    assert_eq!((1, 32), apply_rotation(s.0, Rotation::Left(82)));
    s.apply(Rotation::Left(82));
    assert_eq!((5, 32), apply_rotation(s.0, Rotation::Right(500)));
    s.apply(Rotation::Right(500));
    assert_eq!((5, 32), apply_rotation(s.0, Rotation::Left(500)));
    s.apply(Rotation::Left(500));
}
#[test]
fn test_example_part2() {
    let mut s = State2(50);
    let rotations = [
        Rotation::Left(68),
        Rotation::Left(30),
        Rotation::Right(48),
        Rotation::Left(5),
        Rotation::Right(60),
        Rotation::Left(55),
        Rotation::Left(1),
        Rotation::Left(99),
        Rotation::Right(14),
        Rotation::Left(82),
    ];
    let zeros = s.apply_multiple(rotations.into_iter());
    assert_eq!(zeros, 6);
}

#[derive(Debug)]
struct State1(pub i32);

impl State1 {
    pub fn apply(&mut self, rotation: Rotation) -> bool {
        let (_, remainder) = apply_rotation(self.0, rotation);
        self.0 = remainder;
        self.0 == 0
    }

    pub fn apply_multiple<I: Iterator<Item = Rotation>>(&mut self, iter: I) -> i32 {
        let mut zeros = 0;
        for item in iter {
            if self.apply(item) {
                zeros += 1
            }
        }
        zeros
    }
}

#[derive(Debug)]
struct State2(pub i32);

impl State2 {
    pub fn apply(&mut self, rotation: Rotation) -> u32 {
        let (zeros, remainder) = apply_rotation(self.0, rotation);
        self.0 = remainder;
        zeros
    }

    pub fn apply_multiple<I: Iterator<Item = Rotation>>(&mut self, iter: I) -> u32 {
        let mut zeros = 0;
        for item in iter {
            zeros += self.apply(item);
        }
        zeros
    }
}

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

#[test]
fn test_steps_part1() {
    let mut s = State1(50);
    s.apply(Rotation::Left(68));
    assert_eq!(s.0, 82);
    s.apply(Rotation::Left(30));
    assert_eq!(s.0, 52);
    s.apply(Rotation::Right(48));
    assert_eq!(s.0, 0);
    s.apply(Rotation::Left(5));
    assert_eq!(s.0, 95);
    s.apply(Rotation::Right(60));
    assert_eq!(s.0, 55);
    s.apply(Rotation::Left(55));
    assert_eq!(s.0, 0);
    s.apply(Rotation::Left(1));
    assert_eq!(s.0, 99);
    s.apply(Rotation::Left(99));
    assert_eq!(s.0, 0);
    s.apply(Rotation::Right(14));
    assert_eq!(s.0, 14);
    s.apply(Rotation::Left(82));
    assert_eq!(s.0, 32);
}

#[test]
fn test_multiple_part1() {
    let mut s = State1(50);
    let rotations = [
        Rotation::Left(68),
        Rotation::Left(30),
        Rotation::Right(48),
        Rotation::Left(5),
        Rotation::Right(60),
        Rotation::Left(55),
        Rotation::Left(1),
        Rotation::Left(99),
        Rotation::Right(14),
        Rotation::Left(82),
    ];
    let zeros = s.apply_multiple(rotations.into_iter());
    assert_eq!(zeros, 3);
}

impl FromStr for Rotation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let dir = chars.next().ok_or_else(|| anyhow::anyhow!("Empty line"))?;
        let num: String = chars.collect();
        let num = num.parse::<i32>()?;
        match dir {
            'L' => Ok(Self::Left(num)),
            'R' => Ok(Self::Right(num)),
            _ => Err(anyhow::anyhow!("invalid input")),
        }
    }
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    part1()?;
    part2()?;
    Ok(())
}

fn part1() -> Result<(), anyhow::Error> {
    let mut current_pos = State1(INITIAL_POSITION);
    let input = std::fs::read_to_string("part1.input")?;
    let zeroes = current_pos.apply_multiple(input.lines().map(|line| {
        line.parse::<Rotation>()
            .expect("failed to convert line to rotation")
    }));
    println!("Part 1:");
    println!("Password: {zeroes}");
    Ok(())
}

fn part2() -> Result<(), anyhow::Error> {
    let mut current_pos = State2(INITIAL_POSITION);
    let input = std::fs::read_to_string("part1.input")?;
    let zeroes = current_pos.apply_multiple(input.lines().map(|line| {
        line.parse::<Rotation>()
            .expect("failed to convert line to rotation")
    }));
    println!("Part 2:");
    println!("Password: {zeroes}");
    Ok(())
}
