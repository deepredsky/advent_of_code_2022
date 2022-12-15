use std::fs;
use std::time::Instant;

use itertools::Itertools;

type Input = String;
type ParsedInput = Vec<Vec<usize>>;

fn puzzle_input(path: &str) -> Input {
    fs::read_to_string(path).expect("should read file")
}

fn parse_input(input: &Input) -> ParsedInput {
    input
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

fn solve_part_1(input: &Input) -> usize {
    let part1_start = Instant::now();
    let part1_result = find_visible(input);
    let part1_elapsed = part1_start.elapsed();
    println!("Part 1 answer: {}", part1_result);
    println!("Part 1 time: {:.2?} \n", part1_elapsed);
    part1_result
}

fn solve_part_2(input: &Input) -> usize {
    let part2_start = Instant::now();
    let part2_result = max_scenic_score(input);
    let part2_elapsed = part2_start.elapsed();
    println!("Part 2 answer: {}", part2_result);
    println!("Part 2 time: {:.2?} \n", part2_elapsed);
    part2_result
}

fn neighbors(ri: usize, ci: usize, val: usize, data: &ParsedInput) -> bool {
    let r_len: usize = data.len().try_into().unwrap();
    let c_len: usize = data.first().unwrap().len().try_into().unwrap();

    if ri == 0 || ri == r_len - 1 {
        return true;
    }

    if ci == 0 || ci == c_len - 1 {
        return true;
    }

    let north: Vec<(usize, usize)> = (0..ri).map(|x| (x, ci)).collect();
    let south: Vec<(usize, usize)> = (ri + 1..r_len).map(|x| (x, ci)).collect();
    let west: Vec<(usize, usize)> = (0..ci).map(|x| (ri, x)).collect();
    let east: Vec<(usize, usize)> = (ci + 1..c_len).map(|x| (ri, x)).collect();

    let segments: Vec<_> = vec![east, west, north, south];

    segments
        .iter()
        .any(|segment| segment.iter().all(|(r1, c1)| data[*r1][*c1] < val))
}

fn find_visible(input: &Input) -> usize {
    let mut visible_trees: usize = 0;
    let data = parse_input(input);
    for (ri, row) in data.iter().enumerate() {
        for (ci, col) in row.iter().enumerate() {
            if neighbors(ri, ci, *col, &data) {
                visible_trees += 1
            }
        }
    }

    visible_trees
}

fn max_scenic_score(input: &Input) -> usize {
    let mut scenic_scores: Vec<usize> = vec![];
    let data = parse_input(input);
    for (ri, row) in data.iter().enumerate() {
        for (ci, col) in row.iter().enumerate() {
            let score: usize = scenic_score(ri, ci, *col, &data);
            scenic_scores.push(score);
        }
    }

    *scenic_scores.iter().max().unwrap()
}

fn scenic_score(ri: usize, ci: usize, val: usize, data: &ParsedInput) -> usize {
    let r_len: usize = data.len();
    let c_len: usize = data.first().unwrap().len();

    if ri == 0 || ri == r_len - 1 {
        return 0;
    }

    if ci == 0 || ci == c_len - 1 {
        return 0;
    }

    // segment of trees in each direction
    // ordered by the closeness
    let north: Vec<(usize, usize)> = (0..ri).rev().map(|x| (x, ci)).collect();
    let south: Vec<(usize, usize)> = (ri + 1..r_len).map(|x| (x, ci)).collect();
    let west: Vec<(usize, usize)> = (0..ci).rev().map(|x| (ri, x)).collect();
    let east: Vec<(usize, usize)> = (ci + 1..c_len).map(|x| (ri, x)).collect();

    let segments: Vec<_> = vec![east, west, north, south];

    // Find the first tree going outwards that blocks the view
    // If none found, then the scenic_score is the number of trees until reaching the edge ie
    // length of the segment in this direction
    segments.iter().fold(1, |acc, el| {
        let m = el.iter().find_position(|&(r1, c1)| val <= data[*r1][*c1]);

        let count: usize = match m {
            Some((pos, _)) => pos + 1, // first tree is at 0th position
            None => el.len(),
        };

        acc * count
    })
}

fn main() {
    let input = puzzle_input("data/day-08.txt");
    solve_part_1(&input);
    solve_part_2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part_1_test() {
        let size = find_visible(&SAMPLE.to_string());
        assert_eq!(size, 21)
    }

    #[test]
    fn part_1_test_1() {
        let data = parse_input(&SAMPLE.to_string());
        assert!(neighbors(1, 1, 5, &data));
        assert!(neighbors(1, 2, 5, &data));
        assert!(!neighbors(1, 3, 1, &data));
        assert!(neighbors(2, 1, 5, &data));
    }

    #[test]
    fn part_2_test_1() {
        let data = parse_input(&SAMPLE.to_string());
        assert_eq!(scenic_score(1, 2, 5, &data), 4);
        assert_eq!(scenic_score(3, 2, 5, &data), 8);
    }

    #[test]
    fn part_2_test_2() {
        let max = max_scenic_score(&SAMPLE.to_string());
        assert_eq!(max, 8);
    }
}
