use std::error::Error;

use Hand::*;
use Strategy::*;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read();
    let hands: Vec<(Hand, Strategy)> = input
        .split("\n")
        .map(|line| -> (Hand, Strategy) {
            let mut iter = line.split(' ');
            let hand = match iter.next() {
                Some("A") => Rock,
                Some("B") => Paper,
                Some("C") => Scissor,
                _ => panic!("Couldn't parse hand."),
            };
            let strategy = match iter.next() {
                Some("X") => Lose,
                Some("Y") => Draw,
                Some("Z") => Win,
                _ => panic!("Couldn't parse strategy."),
            };
            (hand, strategy)
        })
        .collect();

    let total: u32 = hands.iter().copied().map(calculate_points).sum();
    dbg!(total);
    Ok(())
}

fn read() -> String {
    std::fs::read_to_string(format!("./data/input.txt")).expect("File not found.")
}

fn calculate_points((other, strategy): (Hand, Strategy)) -> u32 {
    let me: Hand = match (strategy, other) {
        (Lose, Paper) | (Draw, Rock) | (Win, Scissor) => Rock,
        (Lose, Scissor) | (Draw, Paper) | (Win, Rock) => Paper,
        (Lose, Rock) | (Draw, Scissor) | (Win, Paper) => Scissor,
    };

    me as u32 + strategy as u32
}

#[derive(Debug, Copy, Clone)]
enum Strategy {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Debug, Copy, Clone)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}
