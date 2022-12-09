use std::{collections::HashSet, error::Error, str::FromStr, time::Instant};

use Direction::*;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let input = read();
    let instructions = parse(&input);
    const N: usize = 10;
    let mut positions = vec![Position { x: 0, y: 0 }; N];
    let mut visited = HashSet::new();
    visited.insert(positions[N - 1]);
    for Instruction { steps, direction } in instructions {
        match direction {
            Right => {
                for _ in 0..steps {
                    positions[0].x += 1;
                    for i in 1..N {
                        let (head, tail) = (positions[i - 1], positions[i]);
                        let tail = follow_head(head, tail);
                        positions[i] = tail;
                    }
                    visited.insert(positions[N - 1]);
                    // println!("{positions:?}");
                }
            }
            Left => {
                for _ in 0..steps {
                    positions[0].x -= 1;
                    for i in 1..N {
                        let (head, tail) = (positions[i - 1], positions[i]);
                        let tail = follow_head(head, tail);
                        positions[i] = tail;
                    }
                    visited.insert(positions[N - 1]);
                    // println!("{positions:?}");
                }
            }
            Up => {
                for _ in 0..steps {
                    positions[0].y += 1;
                    for i in 1..N {
                        let (head, tail) = (positions[i - 1], positions[i]);
                        let tail = follow_head(head, tail);
                        positions[i] = tail;
                    }
                    visited.insert(positions[N - 1]);
                    // println!("{positions:?}");
                }
            }
            Down => {
                for _ in 0..steps {
                    positions[0].y -= 1;
                    for i in 1..N {
                        let (head, tail) = (positions[i - 1], positions[i]);
                        let tail = follow_head(head, tail);
                        positions[i] = tail;
                    }
                    visited.insert(positions[N - 1]);
                    // println!("{positions:?}");
                }
            }
        }
        // print_pos(&positions);
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

fn print_pos(positions: &[Position]) {
    let xmin: i32 = positions.iter().map(|pos| pos.x).min().unwrap();
    let xmax: i32 = positions.iter().map(|pos| pos.x).max().unwrap();
    let ymin: i32 = positions.iter().map(|pos| pos.y).min().unwrap();
    let ymax: i32 = positions.iter().map(|pos| pos.y).max().unwrap();

    for row in (xmin..=xmax).rev() {
        for col in ymin..=ymax {
            if let Some(index) = positions
                .iter()
                .position(|&p| p == Position { x: row, y: col })
            {
                print!("{}", index);
            } else {
                print!(".");
            }
            print!("\n");
        }
    }
    print!("\n");
}

fn follow_head(head: Position, mut tail: Position) -> Position {
    let (dx, dy) = (head.x - tail.x, head.y - tail.y);

    match (dx, dy) {
        // Outer ring.
        (1, 2) | (2, 2) | (2, 1) => {
            tail.x += 1;
            tail.y += 1
        }
        (2, 0) => tail.x += 1,
        (2, -1) | (2, -2) | (1, -2) => {
            tail.x += 1;
            tail.y -= 1
        }
        (0, -2) => tail.y -= 1,
        (-1, -2) | (-2, -2) | (-2, -1) => {
            tail.x -= 1;
            tail.y -= 1
        }
        (-2, 0) => tail.x -= 1,
        (-2, 1) | (-2, 2) | (-1, 2) => {
            tail.x -= 1;
            tail.y += 1
        }
        (0, 2) => tail.y += 1,
        // Inner square.
        (dx, dy) if dx.abs() <= 1 && dy.abs() <= 1 => {}
        // Outside of 5x5 nearest neighbors.
        (dx, dy) => panic!("(dx, dy) was {:?}. Should be at most 2.", (dx, dy)),
    };
    tail
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
