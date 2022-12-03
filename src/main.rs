use std::{error::Error, collections::HashSet};


fn main() -> Result<(), Box<dyn Error>> {
    let input = read();
    let total: u32 = input
        .lines()
        .map(|line| {
            let len = line.len();
            let (mut first, second) = line.split_at(len / 2);
            let first = HashSet::<char>::from_iter(first.chars());
            let second = HashSet::<char>::from_iter(second.chars());
            let common = first.intersection(&second).next().unwrap();
            calculate_value(common)
        })
        .sum();
    dbg!(total);
    Ok(())
}

fn read() -> String {
    std::fs::read_to_string(format!("./data/input.txt")).expect("File not found.")
}

fn calculate_value(c: &char) -> u32 {
    const CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    CHARS.find(*c).unwrap() as u32 + 1
}