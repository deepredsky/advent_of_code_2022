use parse_display::{Display, FromStr};
use std::fs;
use std::time::Instant;
use array_tool::vec::Uniq;

use itertools::Itertools;

type Input = String;

#[derive(Copy, Clone, Debug, FromStr, Display)]
#[display("{direction} {count}")]
struct Instruction {
    direction: char,
    count: usize,
}

fn puzzle_input(path: &str) -> Input {
    fs::read_to_string(path).expect("should read file")
}

fn parse_input(input: &Input) -> Vec<Instruction> {
    input
        .lines()
        .map(|x| x.parse::<Instruction>().unwrap())
        .collect()
}

fn solve_part_1(input: &Input) -> usize {
let part1_start = Instant::now();
let part1_result = find_positions(input);
let part1_elapsed = part1_start.elapsed();
println!("Part 1 answer: {}", part1_result);
println!("Part 1 time: {:.2?} \n", part1_elapsed);
part1_result
}

// fn solve_part_2(input: &Input) -> usize {
// let part2_start = Instant::now();
// let part2_result = max_scenic_score(input);
// let part2_elapsed = part2_start.elapsed();
// println!("Part 2 answer: {}", part2_result);
// println!("Part 2 time: {:.2?} \n", part2_elapsed);
// part2_result
// }


type Point = (isize, isize);

fn is_adjacent(pos1: Point, pos2: Point) -> bool {
    let (x1, y1) = pos1;
    let (x2, y2) = pos2;

    vec![
        (-1, 1),
        (0, 1),
        (1, 1),

        (-1, 0),
        (0, 0),
        (1, 0),

        (-1, -1),
        (0, -1),
        (1, -1),
    ]
    .iter()
    .any(|(dx, dy)| (x1 + dx == x2) && (y1 + dy == y2))
}

fn find_positions(input: &Input) -> usize {
    let instructions = parse_input(input);

    let mut tail_visited_points: Vec<Point> = vec![(0, 0)];

    // [(00), (00), (00)]
    // [(10), (00), (00)]
    // [(20), (00), (00)]
    // [(20), (10), (00)]
    // [(30), (20), (10)]
    // [(40), (30), (20)]
    // [(41), (30), (20)] -> prev
    // [(42), (41), (30)]

    let mut state: Vec<Point> = vec![(0,0), (0,0)];

    for instruction in instructions {
        let Instruction { direction, count } = instruction;

        match direction {
            'U' => {
                for _ in 0..count {
                    let prev_state = state.clone();

                    let (cur_x, cur_y) = state.first().unwrap();

                    state[0] = (*cur_x, cur_y + 1);

                    // println!("Cur: {:?}, Prev: {:?}", (&cur_x, cur_y), (prev_x, prev_y));
                    if !is_adjacent(*state.first().unwrap(), *state.last().unwrap()) {
                        println!("not adjacent!");
                        state[1] = prev_state[0];
                    }
                    if state.last().unwrap() != prev_state.last().unwrap() {
                        tail_visited_points.push(*state.last().unwrap());
                    }
                }
            }
            'D' => {
                for _ in 0..count {
                    let prev_state = state.clone();

                    let (cur_x, cur_y) = state.first().unwrap();

                    state[0] = (*cur_x, cur_y - 1);

                    // println!("Cur: {:?}, Prev: {:?}", (&cur_x, cur_y), (prev_x, prev_y));
                    if !is_adjacent(*state.first().unwrap(), *state.last().unwrap()) {
                        println!("not adjacent!");
                        state[1] = prev_state[0];
                    }
                    if state.last().unwrap() != prev_state.last().unwrap() {
                        tail_visited_points.push(*state.last().unwrap());
                    }
                }
            }
            'L' => {
                for _ in 0..count {
                    let prev_state = state.clone();

                    let (cur_x, cur_y) = state.first().unwrap();

                    state[0] = (cur_x -1, *cur_y);

                    // println!("Cur: {:?}, Prev: {:?}", (&cur_x, cur_y), (prev_x, prev_y));
                    if !is_adjacent(*state.first().unwrap(), *state.last().unwrap()) {
                        println!("not adjacent!");
                        state[1] = prev_state[0];
                    }
                    if state.last().unwrap() != prev_state.last().unwrap() {
                        tail_visited_points.push(*state.last().unwrap());
                    }
                }
            }
            'R' => {
                for _ in 0..count {
                    let prev_state = state.clone();

                    let (cur_x, cur_y) = state.first().unwrap();

                    state[0] = (cur_x +1, *cur_y);

                    // println!("Cur: {:?}, Prev: {:?}", (&cur_x, cur_y), (prev_x, prev_y));
                    if !is_adjacent(*state.first().unwrap(), *state.last().unwrap()) {
                        println!("not adjacent!");
                        state[1] = prev_state[0];
                    }

                    if state.last().unwrap() != prev_state.last().unwrap() {
                        tail_visited_points.push(*state.last().unwrap());
                    }
                }
            }
            _ => panic!("unknown direction"),
        };
    }

    // dbg!(&tail_visited_points);

    tail_visited_points.unique().len()
}

fn main() {
    let input = puzzle_input("data/day-09.txt");
    solve_part_1(&input);
    // solve_part_2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part_1_test() {
        let size = find_positions(&SAMPLE.to_string());
        assert_eq!(size, 13)
    }
}
