use anyhow::Result;
use std::fs;

use parse_display::{Display, FromStr};
use std::time::Instant;

#[derive(Copy, Clone, Debug, FromStr, Display)]
#[display("{lo}-{hi}")]
struct Section {
    lo: i64,
    hi: i64,
}

impl Section {
    fn contains(&self, other: &Section) -> bool {
        self.lo <= other.lo && self.hi >= other.hi
    }
    fn intersects(&self, other: &Section) -> bool {
        (other.lo >= self.lo && other.lo <= self.hi)
            || (self.lo >= other.lo && self.lo <= other.hi)
    }
}

#[derive(Copy, Clone, Debug, FromStr, Display)]
#[display("{a},{b}")]
struct Input {
    a: Section,
    b: Section,
}

fn main() -> Result<()> {
    let path = "data/day-04.txt";
    let lines = fs::read_to_string(path)
        .expect("works")
        .lines()
        .map(|line| line.parse::<Input>())
        .collect::<Result<Vec<Input>, _>>()?;

    let part1_start = Instant::now();
    let contained = lines
        .iter()
        .filter(|i| i.a.contains(&i.b) || i.b.contains(&i.a))
        .count();
    let part1_time = part1_start.elapsed();
    println!("Part 1 answer: {contained}");
    println!("Part 1 time: {:.2?}", part1_time);

    let part2_start = Instant::now();
    let overlaps = lines.iter().filter(|i| i.a.intersects(&i.b)).count();
    let part2_time = part2_start.elapsed();
    println!("Part 2 answer: {overlaps}");
    println!("Part 2 time: {:.2?}", part2_time);
    Ok(())
}
