use std::error::Error;

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read();
    let mut hands: Vec<(Hand, Hand)> = input
        .split("\n")
        .map(|hand_string| -> (Hand, Hand) {
            hand_string
                .split(' ')
                .map(|hand| match hand {
                    "A" => Hand::Rock,
                    "B" => Hand::Paper,
                    "C" => Hand::Scissor,
                    "X" => Hand::Rock,
                    "Y" => Hand::Paper,
                    "Z" => Hand::Scissor,
                    _ => panic!("Couldn't parse string."),
                })
                .collect_tuple().unwrap()
        })
        .collect();
    
    let total: u32 = hands.iter().copied().map(calculate_points).sum();
    // totals.sort_unstable();
    // let max_3: u32 = totals.iter().rev().take(3).sum();
    // dbg!(hands);
    dbg!(total);
    Ok(())
}

fn read() -> String {
    std::fs::read_to_string(format!("./data/input.txt")).expect("File not found.")
}

fn calculate_points((other, me): (Hand, Hand)) -> u32 {
    use Hand::*;
    let mut total = match (&me, other) {
        (Rock, Scissor) | (Scissor, Paper) | (Paper, Rock) => 6,
        (Rock, Rock) | (Paper, Paper) | (Scissor, Scissor) => 3,
        _ => 0,
    };

    total += match me {
        Rock => 1,
        Paper => 2,
        Scissor => 3,
    };
    total
}

#[derive(Debug, Copy, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}
