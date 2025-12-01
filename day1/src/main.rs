use std::str::FromStr;

const INITIAL_POSITION: i32 = 50;
const MAX_POSITION: i32 = 99;

struct State1(pub i32);

impl State1 {
    pub fn apply(&mut self, rotation: Rotation) -> bool {
        match rotation {
            Rotation::Left(n) => {
                let n = n % (MAX_POSITION + 1);
                if n > self.0 {
                    let extra = n - self.0;
                    self.0 = MAX_POSITION - (extra - 1);
                } else {
                    self.0 -= n;
                }
            }
            Rotation::Right(n) => self.0 = (self.0 + n) % (MAX_POSITION + 1),
        };
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
    part1()?;
    Ok(())
}

fn part1() -> Result<(), anyhow::Error> {
    let mut current_pos = State1(INITIAL_POSITION);
    let input = std::fs::read_to_string("part1.input")?;
    let zeroes = current_pos.apply_multiple(input.lines().map(|line| {
        line.parse::<Rotation>()
            .expect("failed to convert line to rotation")
    }));
    println!("Password: {zeroes}");
    Ok(())
}
