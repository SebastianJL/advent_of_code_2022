use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read();
    let mut totals: Vec<u32> = input
        .split("\n\n")
        .map(|elf_string| {
            elf_string
                .split('\n')
                .map(|number| number.parse::<u32>().expect("Couldn't parse integer."))
                .sum()
        })
        .collect();

    totals.sort_unstable();
    let max_3: u32 = totals.iter().rev().take(3).sum();
    dbg!(max_3);
    Ok(())
}

fn read() -> String {
    std::fs::read_to_string(format!("./data/input.txt")).expect("File not found.")
}
