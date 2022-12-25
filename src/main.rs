use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let input = read();
    let dec_total: u64 = input.lines().map(|line| snafu_to_dec(line)).sum();
    dbg!(dec_total);
    let snafu_total = dec_to_snafu(dec_total);
    dbg!(snafu_total);

    let runtime = start.elapsed();
    dbg!(runtime);
    Ok(())
}

fn read() -> String {
    std::fs::read_to_string("input.txt").expect("File not found.")
}

fn snafu_to_dec(snafu: &str) -> u64 {
    let mut total: i64 = 0;
    let radix: i64 = 5; 
    for (i, c) in snafu.chars().rev().enumerate() {
        total += radix.pow(i as u32) * match c {
            a @ ('0' | '1' | '2') => a.to_digit(radix as u32).unwrap() as i64,
            '=' => -2,
            '-' => -1,
            x => panic!("Didn't expect {x}."),
        }
    }

    total as u64
}

fn dec_to_snafu(dec: u64) -> String {
    let quinten = format_radix(dec, 5);
    dbg!(&quinten);

    let mut snafu = String::new();
    let mut carry = 0;
    for mut c in quinten.chars().rev() {
        let mut tmp_c = c.to_digit(5).unwrap();
        if carry != 0 {
            tmp_c += carry;
            carry = 0;
        }
        if tmp_c > 4 {
            carry += 1;
            tmp_c %= 5;
        }
        c = (tmp_c).to_string().chars().next().unwrap();
        let new_c = match c {
            a @ ('0' | '1' | '2') => a,
            '3' => {
                carry += 1;
                '='
            }
            '4' => {
                carry += 1;
                '-'
            }
            x => panic!("Didn't expect {x}"),
        };
        snafu.push(new_c);
    }

    if carry != 0 {
        snafu.push('1');
    }
    snafu.chars().rev().collect()
}

fn format_radix(mut x: u64, radix: u64) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(char::from_digit(m as u32, radix as u32).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}
