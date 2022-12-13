use array_tool::vec::Uniq;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

type Input = String;

fn parse_input(path: &str) -> Input {
    fs::read_to_string(path).expect("should read file")
}

fn parent_keys(stack: &Vec<&str>) -> Vec<String> {
    (0..stack.len() + 1)
        .map(|n| stack.iter().take(n).join("_"))
        .collect()
}

fn parse_fs(input: Input) -> HashMap<String, usize> {
    let mut fs: HashMap<String, usize> = HashMap::new();
    let mut stack: Vec<&str> = vec![];

    for f in input.split("$ ") {
        if f.is_empty() { continue; }
        let m = f.lines().collect::<Vec<_>>();
        let (head, tail) = m.split_at(1);

        let c = *head.first().unwrap();

        match c {
            "" => println!("empty cmd found"),
            c if c.starts_with("cd") => {
                let tokens: Vec<&str> = c.split(' ').collect();
                match *tokens.last().unwrap() {
                    ".." => {
                        stack.pop();
                    }
                    name => stack.push(name),
                }
            }
            c if c.starts_with("ls") => {
                let mut total_size: usize = 0;
                for &item in tail {
                    let tokens: Vec<&str> = item.split(' ').collect();
                    let file = *tokens.last().unwrap();
                    if *tokens.first().unwrap() == "dir" {
                        // add dir entry
                        // fs.entry(file).or_insert(0);
                    } else {
                        // update parent dir size
                        let size = tokens.first().unwrap().parse::<usize>().unwrap();

                        total_size += size;
                        println!("File: {}, size: {}", file, size)
                    }
                }

                for parent_key in parent_keys(&stack) {
                    fs.entry(parent_key)
                        .and_modify(|size| *size += total_size)
                        .or_insert(total_size);
                }
            }
            _ => panic!("wtf {:?}", c),
        }
    }

    fs
}

fn solve_part_1(input: &Input) -> usize {
    let part1_start = Instant::now();
    let fs = parse_fs(input.to_string());
    let part1_result = fs.iter()
        .filter(|&(_f, s)| *s <= 100000)
        .map(|(_f, s)| *s)
        .sum();
    let part1_elapsed = part1_start.elapsed();
    println!("Part 1 answer: {}", part1_result);
    println!("Part 1 time: {:.2?} \n", part1_elapsed);
    part1_result
}

fn solve_part_2(input: &Input) -> usize {
    let part2_start = Instant::now();
    let fs = parse_fs(input.to_string());
    let total_space: usize = 70000000;
    let required_free_space: usize = 30000000;
    let cur_free_space: usize = total_space - fs.get("/").unwrap();
    let part2_result = fs.iter()
        .filter(|&(_f, s)| cur_free_space + s >= required_free_space)
        .map(|(_f, s)| *s)
        .min()
        .unwrap();
    let part2_elapsed = part2_start.elapsed();
    println!("Part 2 answer: {}", part2_result);
    println!("Part 2 time: {:.2?} \n", part2_elapsed);
    part2_result
}

fn main() {
    let input = parse_input("data/day-07.txt");
    solve_part_1(&input);
    solve_part_2(&input);
}

const INPUT_SAMPLE: &str = "
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_test() {
        let size = solve_part_1(&INPUT_SAMPLE.to_string());
        assert_eq!(size, 95437)
    }

    #[test]
    fn part_2_test() {
        let size = solve_part_2(&INPUT_SAMPLE.to_string());
        assert_eq!(size, 24933642)
    }
}
