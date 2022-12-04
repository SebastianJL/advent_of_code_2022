use std::{error::Error, time::Instant};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let input = read();
    let total: u32 = input
        .lines()
        .map(|line| -> u32 {
            // Split string.
            let Some((min1, max1, min2, max2)) = line.split([',', '-']).next_tuple() else {
                panic!("Couldn't split string");
            };

            // Parse numbers.
            let (min1, max1): (u32, u32) = (min1.parse().unwrap(), max1.parse().unwrap());
            let (min2, max2): (u32, u32) = (min2.parse().unwrap(), max2.parse().unwrap());

            // Test overlap.
            if (min1 <= max2) && (min2 <= max1) {
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
