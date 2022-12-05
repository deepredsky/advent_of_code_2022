use std::fs;
use std::time::Instant;
use parse_display::{Display, FromStr};

type Input = Vec<Instruction>;
type Crate = Vec<char>;

#[derive(Copy, Clone, Debug, FromStr, Display)]
#[display("move {n} from {from} to {to}")]
struct Instruction {
  from: usize,
  to: usize,
  n: usize
}

fn generate_crates() -> Vec<Crate> {
  vec![
  vec!['€'],
  vec!['B', 'Z', 'T'],
  vec!['V', 'H', 'T', 'D', 'N'],
  vec!['B', 'F', 'M', 'D'],
  vec!['L', 'Q', 'V', 'W', 'G', 'J', 'T'],
  vec!['M', 'Q', 'F', 'V', 'P', 'G', 'D', 'W'],
  vec!['S', 'F', 'H', 'G', 'Q', 'Z', 'V'],
  vec!['W', 'C', 'T', 'L', 'R', 'N', 'S', 'Z'],
  vec!['M', 'R', 'N', 'J', 'D', 'W', 'H', 'Z'],
  vec!['S', 'D', 'F', 'L', 'Q', 'M']
]
}

// fn parse_instruction(ins: &str) -> Instruction {
//   let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
  
//   let captures = regex.captures(ins).unwrap();
//   let n = captures[1].parse::<u8>().unwrap();
//   let from_id = captures[2].parse::<u8>().unwrap();
//   let to_id = captures[3].parse::<u8>().unwrap();
//   Instruction { from: from_id, to: to_id, n: n }
// }

fn parse_input(path: &str) -> Input {
    fs::read_to_string(path)
        .expect("should read file")
        .lines()
        .map(|x| x.parse::<Instruction>().unwrap())
        .collect()
}

// fn parse_input_backup(path: &str) -> Input {
//   fs::read_to_string(path)
//       .expect("should read file")
//       .lines()
//       .map(|x| parse_instruction(x))
//       .collect()
// }

fn move_box(crates: Vec<Crate>, from: usize, to: usize, n: usize) -> Vec<Crate> {
  let mut new_crates = crates.clone();
  for _ in 0..n {
    let xbox = new_crates[from].pop().unwrap();
    new_crates[to].push(xbox);
  }
  new_crates
}

fn compute_result(crates: Vec<Crate>) -> Crate {
  let mut result: Crate = vec![];
  for c in crates {
    let letter = c[c.len()-1];
    result.push(letter);
  }
  result
}

fn part_1(input: Input) -> Crate {
  let mut crates = generate_crates();
    for i in input {
      crates = move_box(crates, i.from, i.to, i.n);
    }
  compute_result(crates)
}

// fn part_2(input: Input) -> usize {
//     let mut count = 0;
//     for section_pair in input {
//         if is_overlapping(section_pair) {
//             count += 1
//         }
//     }
//     count
// }

fn solve_part_1(input: Input) {
    let part1_start = Instant::now();
    let part1_result = part_1(input);
    let part1_elapsed = part1_start.elapsed();
    println!("Part 1 answer: {:?}", part1_result);
    println!("Part 1 time: {:.2?} \n", part1_elapsed);
}

// fn solve_part_2(input: Input) {
//     let part2_start = Instant::now();
//     let part2_result = part_2(input);
//     let part2_elapsed = part2_start.elapsed();
//     println!("Part 2 answer: {}", part2_result);
//     println!("Part 2 time: {:.2?}", part2_elapsed);
// }

fn main() {
    //let input = parse_input("data/day-04.txt");
    let input = parse_input("data/day-05.txt");
    println!("input: {:?}", input);
    solve_part_1(input.clone());
    // solve_part_2(input.clone());
}
