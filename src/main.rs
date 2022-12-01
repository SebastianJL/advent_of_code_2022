use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = read();
    input.push('\n');
    let calories = input
        .split('\n')
        .map(|s| s.parse::<u32>().ok());

    let mut totals: Vec<u32> = vec![];
    let mut sum = 0;
    for calory in calories {
        match calory {
            Some(val) => sum += val,
            None => {totals.push(sum); sum = 0}
        }
    }

    totals.sort_unstable();
    let max_3: u32 = totals.iter().rev().take(3).sum();
    dbg!(max_3);
    Ok(())
}

fn read() -> String {
    std::fs::read_to_string(format!("./data/input.txt")).unwrap()
}

