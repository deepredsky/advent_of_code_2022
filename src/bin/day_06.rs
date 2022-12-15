use array_tool::vec::Uniq;
use itertools::Itertools;
use std::fs;
use std::time::Instant;

type Input = String;

fn parse_input(path: &str) -> Input {
    fs::read_to_string(path).expect("should read file")
}

fn solve_part_1(input: &Input) {
    let part1_start = Instant::now();
    let part1_result = find_chars(input, 4);
    let part1_elapsed = part1_start.elapsed();
    println!("Part 1 answer: {}", part1_result);
    println!("Part 1 time: {:.2?} \n", part1_elapsed);
}

fn solve_part_2(input: &Input) {
    let part1_start = Instant::now();
    let part1_result = find_chars(input, 14);
    let part1_elapsed = part1_start.elapsed();
    println!("Part 1 answer: {}", part1_result);
    println!("Part 1 time: {:.2?} \n", part1_elapsed);
}

fn main() {
    let input = parse_input("data/day-06.txt");
    solve_part_1(&input);
    solve_part_2(&input);
}

fn find_chars(signal: &str, win_size: usize) -> usize {
    let (m, _) = signal
        .chars()
        .collect::<Vec<char>>()
        .windows(win_size)
        .find_position(|&x| x.to_vec().is_unique())
        .unwrap();

    m + win_size
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_test_1() {
        assert_eq!(find_chars("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
    }

    #[test]
    fn part_1_test_2() {
        assert_eq!(find_chars("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
    }

    #[test]
    fn part_1_test_3() {
        assert_eq!(find_chars("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
    }

    #[test]
    fn part_1_test_4() {
        assert_eq!(find_chars("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
    }

    #[test]
    fn part_1_test_5() {
        assert_eq!(find_chars("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn part_2_test_1() {
        assert_eq!(find_chars("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
    }

    #[test]
    fn part_2_test_2() {
        assert_eq!(find_chars("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
    }

    #[test]
    fn part_2_test_3() {
        assert_eq!(find_chars("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
    }

    #[test]
    fn part_2_test_4() {
        assert_eq!(find_chars("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
    }

    #[test]
    fn part_2_test_5() {
        assert_eq!(find_chars("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
