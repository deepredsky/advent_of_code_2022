use std::fs;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn parse_input(path: &str) -> Vec<i32> {
    let input: String = fs::read_to_string(path).expect("should read file");
    let new_elves: Vec<i32> = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| {
            let loads_string = x.to_string();
            let loads: Vec<&str> = loads_string.split("\n").collect();
            loads.iter().map(|y| y.parse::<i32>().unwrap()).sum()
        })
        .collect();
    new_elves
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
    println!("Part 2: {}", part_2(elves.clone()));
}
