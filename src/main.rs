use std::{error::Error, time::Instant};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let input = read();
    let (mut stacks, instructions) = parse_input(&input);
    let duration_parse = start.elapsed();
    dbg!(duration_parse);
    
    for Instruction { n, from, to } in instructions {
        let get = stacks.get_mut(from).unwrap();
        let len = get.len();
        let split = get.split_off(len - n);
        let insert = stacks.get_mut(to).unwrap();
        insert.extend(split);
    }

    let mut answer = String::new();
    for stack in stacks {
        answer.push(*stack.last().unwrap());
    }
    dbg!(answer);

    let runtime = start.elapsed();
    dbg!(runtime);
    Ok(())
}

fn read() -> String {
    std::fs::read_to_string(format!("./data/input.txt")).expect("File not found.")
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Instruction {
    n: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let re = Regex::new(r"move (\d{1,}) from (\d{1,}) to (\d{1,})").unwrap();
        let cap = re.captures(input).unwrap();
        let (n, mut from, mut to): (usize, usize, usize) = (
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
        );

        // Adjust 1-based input to 0-based indexing.
        from -= 1;
        to -= 1;
        Instruction { n, from, to }
    }
}
fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let (stacks_str, instructions_str) = input.split_once("\n\n").unwrap();

    // Parse stacks.
    let re = Regex::new(r"\[([A-Z])\]| {3,}").unwrap();
    let mut stack_lines = stacks_str.lines().rev();
    let n_stacks: u32 = stack_lines
        .next()
        .unwrap()
        .split("   ")
        .last()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..n_stacks {
        stacks.push(vec![]);
    }

    for line in stack_lines {
        let mut stack_number = 0;
        for cap in re.captures_iter(line) {
            let outer = &cap[0];
            if outer.starts_with('[') {
                let inner = *(&cap[1].chars().next().unwrap());
                let stack = &mut stacks[stack_number];
                stack.push(inner);
                stack_number += 1;
            } else if outer.starts_with(' ') {
                let num_spaces = outer.len();
                let num_advance = if num_spaces % 2 == 1 {
                    (num_spaces - 1) / 4
                } else {
                    (num_spaces - 1) / 4 + 1
                };
                stack_number += num_advance;
            }
        }
    }

    // Parse instructions.
    let mut instructions = vec![];
    for line in instructions_str.lines() {
        instructions.push(Instruction::from(line));
    }

    (stacks, instructions)
}

#[test]
fn move_from() {
    let input = "move 1 from 2 to 1";
    let instruction = Instruction::from(input);
    assert_eq!(
        instruction,
        Instruction {
            n: 1,
            from: 2,
            to: 1
        }
    );
}
