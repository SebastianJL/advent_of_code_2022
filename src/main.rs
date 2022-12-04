use std::{error::Error, collections::HashSet, time::Instant};

use itertools::Itertools;


fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let input = read();
    let total: u32 = input
        .lines()
        .map(|line| -> u32 {
            let Some((first, second)) = line.split_once(',') else {
                panic!("Couldn't parse line.")
            };
            let Some((min1, max1)) = first.split_once('-') else {
                panic!("Couldn't first elf.")
            };
            let Some((min2, max2)) = second.split_once('-') else {
                panic!("Couldn't second elf.")
            };
            if (min1 <= min2 && max2 <= max1) || (min2 <= min1 && max1 <= max2) {
                1
            } else {
                0
            }
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