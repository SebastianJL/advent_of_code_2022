use std::{error::Error, time::Instant, collections::HashSet};

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let input = read();

    let chars: Vec<char> = input.chars().collect();
    let n_distinct = 14;
    let mut first = 0;
    for i in 0..chars.len() {
        let set = HashSet::<&char>::from_iter(&chars[i..(i+n_distinct)]);
        if set.len() == n_distinct {
            first = i;
            break;
        }
    }
    dbg!(first + n_distinct);

    let runtime = start.elapsed();
    dbg!(runtime);
    Ok(())
}

fn read() -> String {
    std::fs::read_to_string(format!("./data/input.txt")).expect("File not found.")
}