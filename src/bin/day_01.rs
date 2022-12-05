use std::fs;

fn parse_input(path: &str) -> Vec<i32> {
    fs::read_to_string(path)
        .expect("should read file")
        .split("\n\n")
        .map(|x| x.trim().lines().map(|y| y.parse::<i32>().unwrap()).sum())
        .collect()
}

fn part_1(elves: &Vec<i32>) -> i32 {
    *elves.iter().max().unwrap()
}

fn part_2(mut elves: Vec<i32>) -> i32 {
    elves.sort();
    elves.iter().rev().take(3).sum()
}

fn main() {
    let elves = parse_input("data/day-01.txt");
    println!("Part 1: {}", part_1(&elves));
    println!("Part 2: {}", part_2(elves));
}
