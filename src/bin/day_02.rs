use std::fs;
use std::time::Instant;

fn parse_input(path: &str) -> Vec<Vec<String>> {
    let input: String = fs::read_to_string(path).expect("should read file");
    let plays = input
        .lines()
        .map(|line| line.split(' ').map(|x| x.to_string()).collect())
        .collect();
    plays
}

fn shape_score(character: &str) -> i32 {
    match character {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("lol you fucked up"),
    }
}

fn outcome(play: Vec<String>) -> i32 {
    match play.join("").as_str() {
        "AX" => 3,
        "AY" => 6,
        "AZ" => 0,
        "BX" => 0,
        "BY" => 3,
        "BZ" => 6,
        "CX" => 6,
        "CY" => 0,
        "CZ" => 3,
        _ => panic!("round not understood"),
    }
}

fn outcome_score(character: &str) -> i32 {
    match character {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!("lol you fucked up"),
    }
}

fn shape_score_part2(play: Vec<String>) -> i32 {
    match play.join("").as_str() {
        "AX" => 3,
        "AY" => 1,
        "AZ" => 2,
        "BX" => 1,
        "BY" => 2,
        "BZ" => 3,
        "CX" => 2,
        "CY" => 3,
        "CZ" => 1,
        _ => panic!("round not understood"),
    }
}

fn part_1(input: Vec<Vec<String>>) -> i32 {
    input
        .iter()
        .map(|play| outcome(play.clone()) + shape_score(&play[1]))
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

fn part_2(input: Vec<Vec<String>>) -> i32 {
    input
        .iter()
        .map(|play| outcome_score(&play[1]) + shape_score_part2(play.to_vec()))
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

fn solve_part_1(input: Vec<Vec<String>>) {
    let part1_start = Instant::now();
    let part_1_result = part_1(input);
    println!("Part 1 answer: {}", part_1_result);
    println!("Part 1 time: {:.2?} \n", part1_start.elapsed());
}

fn solve_part_2(input: Vec<Vec<String>>) {
    let part2_start = Instant::now();
    let part_2_result = part_2(input);
    println!("Part 2 answer: {}", part_2_result);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
}

fn main() {
    let input = parse_input("data/day-02.txt");
    // println!("input: {:?}", input);
    solve_part_1(input.clone());
    solve_part_2(input.clone());
}
