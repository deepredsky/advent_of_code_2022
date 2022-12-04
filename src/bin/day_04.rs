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
        .map(|x| {
             x.split(",")
             .map(|y| y.to_string()).collect::<Vec<String>>()
            }
        )
        .map(|x| // ["2kk3", "4..5"]
             x.iter().map(|r| convert_to_range(r.to_string())).collect::<SectionPair>()
             )
        .collect()
}

// 1-5
fn convert_to_range(input: String) -> Range<usize> {
    let parsed = input
        .split("-").map(|x| x.to_string().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let a = parsed[0];
    let b = parsed[1];

    if parsed[0] == parsed[1] {
        a..(a+1)
    } else {
        a..(b+1)
    }
}

// "1-5" -> 1..5


fn wat(section_pair: SectionPair) -> bool {
    let a = &section_pair[0];
    let ac = a.clone().collect::<Vec<usize>>();
    let b = &section_pair[1];
    let bc = b.clone().collect::<Vec<usize>>();


    let x = ac.intersect(bc);

    assert_eq!(vec![1].intersect(vec![1,2]), vec![1]);
    assert_eq!(vec![1,2].intersect(vec![1]), vec![1]);

    println!("x {:?}", x);

    x.len() == a.len() || x.len() == b.len()
}

fn part_1(input: Input) -> usize {
    let mut count = 0;
    for section_pair in input {
        if wat(section_pair) {
            count += 1
        }
    }
            count
}

fn part_2(rucksacks: Vec<String>) -> u32 {
    0
}

fn solve_part_1(input: Input) {
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
    //let input = parse_input("data/day-04.txt");
    let input = parse_input("data/day-04.txt");
    // println!("input: {:?}", input);
    solve_part_1(input.clone());
//    solve_part_2(input.clone());
}
