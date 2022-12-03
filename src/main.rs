use std::{error::Error, collections::HashSet, time::Instant};

use itertools::Itertools;


fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let input = read();
    let total: u32 = input
        .lines()
        .batching(|it| -> Option<u32> {
            let (elf1, elf2, elf3) = (it.next()?, it.next()?, it.next()?);
            type Set = HashSet::<char>;
            let elf1 = Set::from_iter(elf1.chars());
            let elf2 = Set::from_iter(elf2.chars());
            let elf3 = Set::from_iter(elf3.chars());
            let common = elf1.intersection(&elf2).copied().collect::<Set>();
            let common = common.intersection(&elf3).next().expect("Couldn't find common letter.");
            Some(calculate_value(*common))
        })
        .sum();
    let runtime = start.elapsed();
    dbg!(total);
    dbg!(runtime);
    Ok(())
}

fn read() -> String {
    std::fs::read_to_string(format!("./data/input.txt")).expect("File not found.")
}

fn calculate_value(c: char) -> u32 {
    const CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    CHARS.find(c).unwrap() as u32 + 1
}