use array_tool::vec::Intersect;
use std::fs;
use std::ops::Range;
use std::time::Instant;

type SectionPair = Vec<Range<usize>>;
type Input = Vec<SectionPair>;

// [
// [
// (1..2), (1..4)
// ]
// ]
fn parse_input(path: &str) -> Vec<SectionPair> {
    fs::read_to_string(path)
        .expect("should read file")
        .lines()
        .map(|x| x.to_string())
        .map(|x| x.split(",").map(|y| y.to_string()).collect::<Vec<String>>())
        .map(|x| // ["2kk3", "4..5"]
             x.iter().map(|r| convert_to_range(r.to_string())).collect::<SectionPair>())
        .collect()
}

// 1-5
fn convert_to_range(input: String) -> Range<usize> {
    let parsed = input
        .split("-")
        .map(|x| x.to_string().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let a = parsed[0];
    let b = parsed[1];

    a..(b + 1)
}

fn convert_range_to_vec(pair: Range<usize>) -> Vec<usize> {
    pair.collect()
}

fn is_contained(section_pair: SectionPair) -> bool {
    let a = convert_range_to_vec(section_pair[0].clone());
    let b = convert_range_to_vec(section_pair[1].clone());
    let x = a.intersect(b.clone()).len();

    x == a.len() || x == b.len()
}

fn is_overlapping(section_pair: SectionPair) -> bool {
    let a = convert_range_to_vec(section_pair[0].clone());
    let b = convert_range_to_vec(section_pair[1].clone());
    let x = a.intersect(b.clone()).len();

    x > 0
}

fn part_1(input: Input) -> usize {
    let mut count = 0;
    for section_pair in input {
        if is_contained(section_pair) {
            count += 1
        }
    }
    count
}

fn part_2(input: Input) -> usize {
    let mut count = 0;
    for section_pair in input {
        if is_overlapping(section_pair) {
            count += 1
        }
    }
    count
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
    let input = parse_input("data/day-04.txt");
    // println!("input: {:?}", input);
    solve_part_1(input.clone());
    solve_part_2(input.clone());
}
