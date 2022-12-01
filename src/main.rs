use std::error::Error;

use csv::ReaderBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b' ')
        .has_headers(false)
        .from_path("./data/input.txt")?;
    for result in reader.records() {
        let record = result?;
        dbg!(&record.get(1));
        dbg!(&record);
        for i in record.iter() {
            dbg!(i);
        }
    }
    Ok(())
}

fn read() -> String {
    std::fs::read_to_string(format!("./data/input.txt")).unwrap()
}

