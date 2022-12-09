use std::{error::Error, str::FromStr, time::Instant, collections::HashSet};

use Direction::*;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let input = read();
    let instructions = parse(&input);
    let (mut head, mut tail) = (Position { x: 0, y: 0 }, Position { x: 0, y: 0 });

    let mut visited = HashSet::new();
    visited.insert(tail);
    for Instruction{steps, direction} in instructions {
        match direction {
            Right => {
                for _ in 0..steps {
                    let (dx, dy) = (tail.x - head.x, tail.y - head.y);
                    match (dx, dy) {
                        (-1, _) => {
                            (tail.x, tail.y) = (head.x, head.y);
                            visited.insert(tail);
                            head.x += 1
                        }
                        (dx, dy) if dx.abs() <= 1 && dy.abs() <= 1 => head.x += 1,
                        (dx, dy) => panic!("(dx, dy) was {:?}. Should be at most 1.", (dx, dy)),
                    }
                    // println!("T: {:?}, H: {:?}", tail, head);
                }
            }
            Left => {
                for _ in 0..steps {
                    let (dx, dy) = (tail.x - head.x, tail.y - head.y);
                    match (dx, dy) {
                        (1, _) => {
                            (tail.x, tail.y) = (head.x, head.y);
                            visited.insert(tail);
                            head.x -= 1
                        }
                        (dx, dy) if dx.abs() <= 1 && dy.abs() <= 1 => head.x -= 1,
                        (dx, dy) => panic!("(dx, dy) was {:?}. Should be at most 1.", (dx, dy)),
                    }
                    // println!("T: {:?}, H: {:?}", tail, head);
                }
            },
            Up => {
                for _ in 0..steps {
                    let (dx, dy) = (tail.x - head.x, tail.y - head.y);
                    match (dx, dy) {
                        (_, -1) => {
                            (tail.x, tail.y) = (head.x, head.y);
                            visited.insert(tail);
                            head.y += 1
                        }
                        (dx, dy) if dx.abs() <= 1 && dy.abs() <= 1 => head.y += 1,
                        (dx, dy) => panic!("(dx, dy) was {:?}. Should be at most 1.", (dx, dy)),
                    }
                    // println!("T: {:?}, H: {:?}", tail, head);
                }
            },
            Down => {
                for _ in 0..steps {
                    let (dx, dy) = (tail.x - head.x, tail.y - head.y);
                    match (dx, dy) {
                        (_, 1) => {
                            (tail.x, tail.y) = (head.x, head.y);
                            visited.insert(tail);
                            head.y -= 1
                        }
                        (dx, dy) if dx.abs() <= 1 && dy.abs() <= 1 => head.y -= 1,
                        (dx, dy) => panic!("(dx, dy) was {:?}. Should be at most 1.", (dx, dy)),
                    }
                    // println!("T: {:?}, H: {:?}", tail, head);
                }
            },
        }
    }

    dbg!(visited.len());

    let runtime = start.elapsed();
    dbg!(runtime);
    Ok(())
}

fn read() -> String {
    std::fs::read_to_string("input.txt").expect("File not found.")
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct Instruction {
    steps: usize,
    direction: Direction,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, steps) = s.split_once(' ').ok_or("Couldn't split.")?;
        let steps = steps.parse().or(Err("Couldn't parse distance."))?;
        let direction = match direction {
            "L" => Left,
            "R" => Right,
            "U" => Up,
            "D" => Down,
            _ => Err("Invalid direction.")?,
        };
        Ok(Instruction { steps, direction })
    }
}
