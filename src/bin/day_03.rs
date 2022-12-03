use array_tool::vec::Intersect;
use std::fs;
use std::time::Instant;

fn parse_input(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("should read file")
        .lines()
        .map(|x| x.to_string())
        .collect()
}

fn score_letter(letter: char) -> u32 {
    if letter.is_lowercase() {
        (letter as u32) - 96
    } else {
        (letter as u32) - 38
    }
}

fn part_1(rucksacks: Vec<String>) -> u32 {
    rucksacks
        .iter()
        .map(|rucksack| {
            let middle_i = rucksack.len() / 2;
            let first_half = &rucksack[0..middle_i];
            let last_half = &rucksack[(middle_i)..];
            let first_chars = first_half.chars().collect::<Vec<char>>();
            let last_chars = last_half.chars().collect::<Vec<char>>();
            let common_letters = first_chars.intersect(last_chars);
            let letter = common_letters[0];
            score_letter(letter)
        })
        .sum()
}

fn part_2(rucksacks: Vec<String>) -> u32 {
    rucksacks
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
        .chunks(3)
        .map(|group| {
            let common = group.iter().cloned().reduce(|acc, elf| acc.intersect(elf));
            let letters = common.unwrap();
            let letter = letters[0];
            score_letter(letter)
        })
        .sum()
}

fn solve_part_1(input: Vec<String>) {
    let part1_start = Instant::now();
    let part_1_result = part_1(input);
    println!("Part 1 answer: {}", part_1_result);
    println!("Part 1 time: {:.2?} \n", part1_start.elapsed());
}

fn solve_part_2(input: Vec<String>) {
    let part2_start = Instant::now();
    let part_2_result = part_2(input);
    println!("Part 2 answer: {}", part_2_result);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
}

fn main() {
    let input = parse_input("data/day-03.txt");
    // println!("input: {:?}", input);
    solve_part_1(input.clone());
    solve_part_2(input.clone());
}
