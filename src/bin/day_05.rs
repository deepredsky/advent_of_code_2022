use parse_display::{Display, FromStr};
use std::fs;
use std::time::Instant;

type Input = Vec<Instruction>;
type Crate = Vec<char>;

#[derive(Copy, Clone, Debug, FromStr, Display)]
#[display("move {n} from {from} to {to}")]
struct Instruction {
    from: usize,
    to: usize,
    n: usize,
}

fn generate_crates() -> Vec<Crate> {
    vec![
        vec!['€'],
        vec!['B', 'Z', 'T'],
        vec!['V', 'H', 'T', 'D', 'N'],
        vec!['B', 'F', 'M', 'D'],
        vec!['T', 'J', 'G', 'W', 'V', 'Q', 'L'],
        vec!['W', 'D', 'G', 'P', 'V', 'F', 'Q', 'M'],
        vec!['V', 'Z', 'Q', 'G', 'H', 'F', 'S'],
        vec!['Z', 'S', 'N', 'R', 'L', 'T', 'C', 'W'],
        vec!['Z', 'H', 'W', 'D', 'J', 'N', 'R', 'M'],
        vec!['M', 'Q', 'L', 'F', 'D', 'S'],
    ]
}

fn parse_input(path: &str) -> Input {
    fs::read_to_string(path)
        .expect("should read file")
        .lines()
        .map(|x| x.parse::<Instruction>().unwrap())
        .collect()
}

fn move_box(crates: Vec<Crate>, from: usize, to: usize, n: usize) -> Vec<Crate> {
    let mut new_crates = crates.clone();
    for _ in 0..n {
        let xbox = new_crates[from].pop().unwrap();
        new_crates[to].push(xbox);
    }
    new_crates
}

fn move_box_2(crates: Vec<Crate>, from: usize, to: usize, n: usize) -> Vec<Crate> {
    let mut new_crates = crates.clone();
    let mut boxes = vec![];
    for _ in 0..n {
        let xbox = new_crates[from].pop().unwrap();
        boxes.insert(0, xbox);
    }
    new_crates[to].append(&mut boxes);
    new_crates
}

fn compute_result(crates: Vec<Crate>) -> String {
    let mut result: Crate = vec![];
    for c in crates {
        let letter = c[c.len() - 1];
        result.push(letter);
    }
    result[1..].iter().collect::<String>()
}

fn part_1(input: Input) -> String {
    let mut crates = generate_crates();
    for i in input {
        crates = move_box(crates, i.from, i.to, i.n);
    }
    compute_result(crates)
}

fn part_2(input: Input) -> String {
    let mut crates = generate_crates();
    for i in input {
        crates = move_box_2(crates, i.from, i.to, i.n);
    }
    compute_result(crates)
}

fn solve_part_1(input: Input) {
    let part1_start = Instant::now();
    let part1_result = part_1(input);
    let part1_elapsed = part1_start.elapsed();
    println!("Part 1 answer: {}", part1_result);
    println!("Part 1 time: {:.2?} \n", part1_elapsed);
}

fn solve_part_2(input: Input) {
    let part2_start = Instant::now();
    let part2_result = part_2(input);
    let part2_elapsed = part2_start.elapsed();
    println!("Part 2 answer: {}", part2_result);
    println!("Part 2 time: {:.2?}", part2_elapsed);
}

fn main() {
    //let input = parse_input("data/day-04.txt");
    let input = parse_input("data/day-05.txt");
    // println!("input: {:?}", input);
    solve_part_1(input.clone());
    solve_part_2(input.clone());
}
