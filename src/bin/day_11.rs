use array_tool::vec::Shift;
use regex::Regex;
use std::fs;
use std::time::Instant;

type Input = String;

#[derive(Clone, Debug)]
struct Operation {
    x: Op,
    operator: String,
    y: Op,
}

#[derive(Clone, Debug)]
struct Monkey {
    starting_items: Vec<u64>,
    operation: Operation,
    div_by: u64,
    div_by_result: (u64, u64),
}

fn puzzle_input(path: &str) -> Input {
    fs::read_to_string(path).expect("should read file")
}

fn extract_starting_items(line: &String) -> Vec<u64> {
    let re = Regex::new(r"\d+").unwrap();

    re.find_iter(&line)
        .map(|mat| mat.as_str().parse::<u64>().unwrap())
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
enum Op {
    Val(u64),
    Old,
}

fn extract_operation(line: &String) -> Operation {
    let x = line.trim().split(" = ").last().unwrap();
    let y: Vec<String> = x.split(" ").map(|t| t.to_string()).collect();

    let a = match y[0].as_str() {
        "old" => Op::Old,
        v => Op::Val(v.parse::<u64>().unwrap()),
    };

    let b = match y[2].as_str() {
        "old" => Op::Old,
        v => Op::Val(v.parse::<u64>().unwrap()),
    };

    Operation {
        x: a,
        operator: y[1].clone(),
        y: b,
    }
}

fn extract_number(line: &String) -> u64 {
    let re = Regex::new(r"\d+").unwrap();

    let m = re
        .find_iter(&line)
        .map(|mat| mat.as_str().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    *m.first().unwrap()
}

fn parse_input(input: &Input) -> usize {
    let monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|group| {
            let x = group
                .lines()
                .map(|f| f.to_string())
                .collect::<Vec<String>>();

            let starting_items = extract_starting_items(&x[1]);
            let operation = extract_operation(&x[2]);
            let divisible = extract_number(&x[3]);
            let divisible_x = extract_number(&x[4]);
            let divisible_y = extract_number(&x[5]);

            Monkey {
                starting_items,
                operation,
                div_by: divisible,
                div_by_result: (divisible_x, divisible_y),
            }
        })
        .collect::<Vec<Monkey>>();

    let mut monkey_items: Vec<Vec<u64>> =
        monkeys.iter().map(|x| x.starting_items.clone()).collect();
    let mut monkey_counts = vec![0; monkeys.len()];

    for _ in 0..20 {
        for (i, monkey) in monkeys.iter().enumerate() {
            monkey_counts[i] = monkey_counts[i] + monkey_items[i].len();

            while monkey_items[i].len() > 0 {
                let item = monkey_items[i].shift().unwrap();
                let worry = calculate_worry_level(item, &monkey.operation);
                let worry = worry / 3;

                let target = if worry % monkey.div_by == 0 {
                    monkey.div_by_result.0 as usize
                } else {
                    monkey.div_by_result.1 as usize
                };

                let mut x = monkey_items[target].clone();
                x.push(worry);
                monkey_items[target] = x;
            }
        }
    }

    monkey_counts.sort();
    monkey_counts.reverse();

    monkey_counts[0] * monkey_counts[1]
}

fn parse_input2(input: &Input) -> usize {
    let monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|group| {
            let x = group
                .lines()
                .map(|f| f.to_string())
                .collect::<Vec<String>>();

            let starting_items = extract_starting_items(&x[1]);
            let operation = extract_operation(&x[2]);
            let divisible = extract_number(&x[3]);
            let divisible_x = extract_number(&x[4]);
            let divisible_y = extract_number(&x[5]);

            Monkey {
                starting_items,
                operation,
                div_by: divisible,
                div_by_result: (divisible_x, divisible_y),
            }
        })
        .collect::<Vec<Monkey>>();

    let mut monkey_items: Vec<Vec<u64>> =
        monkeys.iter().map(|x| x.starting_items.clone()).collect();
    let mut monkey_counts = vec![0; monkeys.len()];
    let all: u64 = monkeys.iter().map(|x| x.div_by).product();

    for _ in 0..10000 {
        for (i, monkey) in monkeys.iter().enumerate() {
            monkey_counts[i] = monkey_counts[i] + monkey_items[i].len();

            while monkey_items[i].len() > 0 {
                let item = monkey_items[i].shift().unwrap();
                let worry = calculate_worry_level(item, &monkey.operation);

                let target = if worry % monkey.div_by == 0 {
                    monkey.div_by_result.0 as usize
                } else {
                    monkey.div_by_result.1 as usize
                };

                let mut x = monkey_items[target].clone();
                x.push(worry % all);
                monkey_items[target] = x;
            }
        }
    }

    monkey_counts.sort();
    monkey_counts.reverse();

    monkey_counts[0] * monkey_counts[1]
}

fn calculate_worry_level(item: u64, operation: &Operation) -> u64 {
    let Operation { x, operator, y } = operation;
    let numbers = match (x, y) {
        (Op::Old, Op::Old) => (item, item),
        (Op::Old, Op::Val(y)) => (item, *y),
        (Op::Val(y), Op::Old) => (*y, item),
        _ => panic!("onhuonu"),
    };

    match operator.as_str() {
        "*" => numbers.0 * numbers.1,
        "+" => numbers.0 + numbers.1,
        "-" => numbers.0 - numbers.1,
        _ => panic!("oeuau"),
    }
}

fn solve_part_1(input: &Input) -> usize {
    let part1_start = Instant::now();
    let part1_result = parse_input(input);

    let part1_elapsed = part1_start.elapsed();
    println!("Part 1 answer: {:?}", part1_result);
    println!("Part 1 time: {:.2?} \n", part1_elapsed);
    part1_result
}

fn solve_part_2(input: &Input) -> usize {
    let part2_start = Instant::now();
    let part2_result = parse_input2(input);
    let part2_elapsed = part2_start.elapsed();
    println!("Part 2 answer: {}", part2_result);
    println!("Part 2 time: {:.2?} \n", part2_elapsed);
    part2_result
}

fn main() {
    let input = puzzle_input("data/day_11.txt");
    solve_part_1(&input);
    solve_part_2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part_1_test() {
        assert_eq!(parse_input(&SAMPLE.to_string()), 10605);
    }
    #[test]
    fn extract_starting_items_test() {
        assert_eq!(
            extract_starting_items(&" Starting items: 79, 60, 97 ".to_string()),
            vec![79, 60, 97]
        );
    }

    #[test]
    fn extract_operation_test() {
        let x = extract_operation(&" Operation: new = old + 3".to_string());

        assert_eq!(x.x, Op::Old);
        assert_eq!(x.y, Op::Val(3));
    }

    #[test]
    fn part_2_test() {
        assert_eq!(parse_input2(&SAMPLE.to_string()), 2713310158);
    }
}
