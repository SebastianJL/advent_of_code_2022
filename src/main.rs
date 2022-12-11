use core::panic;
use std::{error::Error, time::Instant, vec};

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let input = read();
    let nums = parse(&input);
    dbg!(&nums);

    let total: i32 = (20..=220)
        .step_by(40)
        .map(|i| {
            let sum = nums[0..i].iter().sum::<i32>() + 1;
            dbg!(i, sum);
            sum * i as i32
        })
        .sum();
    dbg!(total);

    let runtime = start.elapsed();
    dbg!(runtime);
    Ok(())
}

fn read() -> String {
    std::fs::read_to_string("input.txt").expect("File not found.")
}

fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .flat_map(|line| {
            let mut iter = line.split(' ');
            let n = match iter.next() {
                Some("noop") => vec![0],
                Some("addx") => vec![0, iter.next().unwrap().parse().unwrap()],
                _ => panic!("Shouldn't happen."),
            };
            n
        })
        .collect()
}
