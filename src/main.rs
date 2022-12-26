use std::{error::Error, time::Instant};

use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::line_ending, multi::separated_list1,
    sequence::separated_pair, IResult,
};

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let input = read();
    let instructions = parse_instructions(&input);
    // dbg!(&instructions);

    let mut cpu = Cpu::from(instructions);
    cpu.x = 1;
    let total = cpu.run();
    dbg!(total);

    let runtime = start.elapsed();
    dbg!(runtime);
    Ok(())
}

fn read() -> String {
    std::fs::read_to_string("input.txt").expect("File not found.")
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let (_, instructions) = separated_list1(line_ending, parse_instruction)(input).unwrap();
    instructions
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, instruction) = alt((parse_addx, parse_noop))(input)?;
    Ok((input, instruction))
}

fn parse_addx(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, num)) =
        separated_pair(tag("addx"), tag(" "), nom::character::complete::i32)(input)?;
    Ok((input, Instruction::Addx(num)))
}

fn parse_noop(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Instruction::Noop))
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Noop,
    Addx(i32),
}

struct Cpu {
    state: CpuState,
    instructions: Vec<Instruction>,
    x: i32,
    counter: u32,
}

#[derive(PartialEq, Eq, Debug)]
enum CpuState {
    ReadInstruction,
    Addx(i32),
    Finished,
}

impl From<Vec<Instruction>> for Cpu {
    fn from(instructions: Vec<Instruction>) -> Self {
        Cpu {
            state: CpuState::ReadInstruction,
            instructions,
            x: 0,
            counter: 0,
        }
    }
}

impl Cpu {
    fn run(&mut self) -> i32 {
        use CpuState::*;

        self.counter = 0;
        let mut iter = self.instructions.iter();

        let cycles: Vec<_> = (20..=220).step_by(40).collect();
        let mut total = 0;

        while self.state != Finished {
            self.counter += 1;

            if cycles.contains(&self.counter) {
                total += self.counter as i32 * self.x;
                println!("{} {} {:?}", self.counter, self.x, self.state);
            }

            let new_state = match self.state {
                ReadInstruction => match iter.next() {
                    None => Finished,
                    Some(Instruction::Noop) => ReadInstruction,
                    Some(Instruction::Addx(num)) => Addx(*num),
                },
                Addx(num) => {
                    self.x += num;
                    ReadInstruction
                }
                Finished => Finished,
            };

            self.state = new_state;
        }

        total
    }
}
