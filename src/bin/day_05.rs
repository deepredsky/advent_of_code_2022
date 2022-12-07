use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::fs;
use std::time::Instant;

type Crate = Vec<char>;

#[derive(Copy, Clone, Debug, FromStr, Display)]
#[display("move {n} from {from} to {to}")]
struct Instruction {
    from: usize,
    to: usize,
    n: usize,
}

struct Input {
    instructions: Vec<Instruction>,
    crates: Vec<Crate>,
}

impl Clone for Input {
    fn clone(&self) -> Self {
        Input {
            instructions: self.instructions.clone(),
            crates: self.crates.clone(),
        }
    }
}

fn parse_crates(input: &str) -> Vec<Vec<char>> {
    let mut lines: Vec<String> = input
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    lines.pop();

    let y: Vec<Vec<Option<char>>> = lines
        .iter()
        .rev()
        .map(|line| parse_crate_line(line))
        .collect();

    matrix_transpose(y)
}

fn parse_crate_line(line: &str) -> Vec<Option<char>> {
    line.chars()
        .chunks(4)
        .into_iter()
        .map(|mut f| f.find(|&y| y.is_alphabetic()))
        .collect::<Vec<Option<char>>>()
}

fn parse_input(path: &str) -> Input {
    let contents: Vec<String> = fs::read_to_string(path)
        .expect("should read file")
        .split("\n\n")
        .map(|x| x.to_string())
        .collect();

    let crates_input: String = contents.first().unwrap().to_string();
    let mut crates = parse_crates(&crates_input);
    crates.insert(0, vec!['X']); // fake the 0th item

    let instructions: Vec<Instruction> = contents
        .last()
        .unwrap()
        .to_string()
        .lines()
        .map(|x| x.parse::<Instruction>().unwrap())
        .collect();

    Input {
        crates,
        instructions,
    }
}

fn move_box(mut crates: Vec<Crate>, from: usize, to: usize, n: usize) -> Vec<Crate> {
    for _ in 0..n {
        let xbox = crates[from].pop().unwrap();
        crates[to].push(xbox);
    }
    crates
}

fn move_box_2(mut crates: Vec<Crate>, from: usize, to: usize, n: usize) -> Vec<Crate> {
    let mut boxes = vec![];
    for _ in 0..n {
        let xbox = crates[from].pop().unwrap();
        boxes.insert(0, xbox);
    }
    crates[to].append(&mut boxes);
    crates
}

fn compute_result(crates: Vec<Crate>) -> String {
    let mut result: Crate = vec![];
    for c in crates {
        let letter = c[c.len() - 1];
        result.push(letter);
    }
    result[1..]
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
}

fn part_1(input: Input) -> String {
    let mut crates = input.crates;
    for i in input.instructions {
        crates = move_box(crates, i.from, i.to, i.n);
    }
    compute_result(crates)
}

fn part_2(input: Input) -> String {
    let mut crates = input.crates;
    for i in input.instructions {
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

fn matrix_transpose(m: Vec<Vec<Option<char>>>) -> Vec<Vec<char>> {
    let mut t = vec![Vec::with_capacity(m.len()); m[0].len()];
    for r in m {
        for i in 0..r.len() {
            if r[i].is_some() {
                t[i].push(r[i].unwrap());
            }
        }
    }
    t
}

fn main() {
    let input = parse_input("data/day-05.txt");
    //println!("input: {:?}", input);
    solve_part_1(input.clone());
    solve_part_2(input.clone());
}
