use parse_display::{Display, FromStr};
use std::fs;
use std::time::Instant;

type Input = String;

#[derive(Copy, Clone, Debug, FromStr, Display)]
enum Instruction {
    #[display("noop")]
    Noop,
    #[display("addx {val}")]
    Add { val: isize },
}

fn puzzle_input(path: &str) -> Input {
    fs::read_to_string(path).expect("should read file")
}

fn parse_input(input: &Input) -> Vec<Result<Instruction, parse_display::ParseError>> {
    input.lines().map(|x| x.parse()).collect()
}

fn solve_part_1(input: &Input) -> isize {
    let part1_start = Instant::now();
    let result = run_program(input);

    let part1_result = vec![
        20 * result[20],
        60 * result[60],
        100 * result[100],
        140 * result[140],
        180 * result[180],
        220 * result[220],
    ]
    .iter()
    .sum();

    let part1_elapsed = part1_start.elapsed();
    println!("Part 1 answer: {:?}", part1_result);
    println!("Part 1 time: {:.2?} \n", part1_elapsed);
    part1_result
}

// fn solve_part_2(input: &Input) -> usize {
//     let part2_start = Instant::now();
//     let part2_result = find_positions(input, 10);
//     let part2_elapsed = part2_start.elapsed();
//     println!("Part 1 answer: {}", part2_result);
//     println!("Part 1 time: {:.2?} \n", part2_elapsed);
//     part2_result
// }

// start cycle 1    noop             1
// end cycle 1      noop             1
// start cycle 2    add 3            1
// end cycle 2                       1
// start cycle 3                     1
// end cycle 3                       4
// start cycle 4    add -5           4
// end cycle 4                       4
// start cycle 5                     4
// end cycle 5                       -1
//
fn run_program(input: &Input) -> Vec<isize> {
    let instructions = parse_input(input);
    let mut process_list: Vec<Option<isize>> = vec![];

    for instruction in instructions {
        match instruction {
            Ok(Instruction::Noop) => {
                process_list.push(None);
            }
            Ok(Instruction::Add { val }) => {
                process_list.push(None);
                process_list.push(Some(val));
            }
            Err(_) => panic!("error parsing"),
        }
    }

    let mut cur_val: isize = 1;
    let mut result: Vec<isize> = vec![1];
    let mut pixels: Vec<char> = vec![];

    for (cur_pixel_pos, val) in (0_isize..).zip(process_list.into_iter()) {
        // start

        // during
        result.push(cur_val);

        let cur_sprite: [isize; 3] = [cur_val + 1, cur_val, cur_val - 1];

        if cur_pixel_pos > 19 {
            println!("{cur_pixel_pos}, {cur_sprite:?}");
        }

        if cur_sprite.iter().any(|v| cur_pixel_pos % 40 == *v) {
            pixels.push('#');
        } else {
            pixels.push(' ');
        }

        // end
        if let Some(x) = val {
            cur_val += x;
        }
    }

    result.push(cur_val);

    for (i, x) in pixels.iter().enumerate() {
        if i % 40 == 0 {
            println!();
        }

        print!("{}", x);
    }

    result
}

fn main() {
    let input = puzzle_input("data/day-10.txt");
    solve_part_1(&input);
    // solve_part_2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "noop
addx 3
addx -5";
    const SAMPLE_2: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part_1_test() {
        let val = run_program(&SAMPLE.to_string());
        assert_eq!(val[5], 4);
        assert_eq!(val[6], -1);
    }

    #[test]
    fn part_1_test_1() {
        let val = run_program(&SAMPLE_2.to_string());
        assert_eq!(val[20], 21);
        assert_eq!(val[60], 19);
        assert_eq!(val[100], 18);
        assert_eq!(val[140], 21);
        assert_eq!(val[180], 16);
        assert_eq!(val[220], 18);
    }

    #[test]
    fn part_1_test_2() {
        let val = solve_part_1(&SAMPLE_2.to_string());
        assert_eq!(val, 13140);
        assert_eq!(val, 13141);
    }
}
