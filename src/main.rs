use std::{collections::HashMap, error::Error, time::Instant};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace1},
    multi::separated_list0,
    IResult,
};

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let input = read();
    let jobs = parse(&input).unwrap();

    let result = yells("root", &jobs);
    dbg!(result);

    let runtime = start.elapsed();
    dbg!(runtime);
    Ok(())
}

fn read() -> String {
    std::fs::read_to_string("input.txt").expect("File not found.")
}

fn yells(monkey: &str, jobs: &HashMap<&str, Job>) -> i64 {
    let job = jobs.get(monkey).unwrap();

    match job {
        &Job::Val(val) => val,
        &Job::Op(op) => match op {
            Operation::Add(l, r) => yells(l, jobs) + yells(r, jobs),
            Operation::Sub(l, r) => yells(l, jobs) - yells(r, jobs),
            Operation::Mul(l, r) => yells(l, jobs) * yells(r, jobs),
            Operation::Div(l, r) => yells(l, jobs) / yells(r, jobs),
        },
    }
}

fn operation(input: &str) -> IResult<&str, Job> {
    let (input, operand1) = alpha1(input)?;
    let (input, _) = multispace1(input)?;
    let (input, binary_op) = alt((tag("+"), tag("-"), tag("/"), tag("*")))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, operand2) = alpha1(input)?;

    let operation = match binary_op {
        "+" => Operation::Add(operand1, operand2),
        "-" => Operation::Sub(operand1, operand2),
        "*" => Operation::Mul(operand1, operand2),
        "/" => Operation::Div(operand1, operand2),
        c => panic!("Invalid binary operator {c}"),
    };

    Ok((input, Job::Op(operation)))
}

fn value(input: &str) -> IResult<&str, Job> {
    let (input, val) = nom::character::complete::i64(input)?;
    Ok((input, Job::Val(val)))
}

fn job(input: &str) -> IResult<&str, Job> {
    let (input, job) = alt((value, operation))(input)?;
    Ok((input, job))
}

fn parse_line(input: &str) -> IResult<&str, (&str, Job)> {
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, value) = job(input)?;

    Ok((input, (name, value)))
}

fn parse(input: &str) -> Result<HashMap<&str, Job>, String> {
    let (input, monkeys) = match separated_list0(tag("\n"), parse_line)(input) {
        Ok(monkeys) => monkeys,
        Err(_) => Err("bla")?,
    };
    if !input.is_empty() {
        Err("Couldn't parse entire input. Residual {input}")?;
    }
    Ok(monkeys.into_iter().collect())
}

#[derive(Debug, Copy, Clone)]
enum Operation<'a> {
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
}

#[derive(Debug, Copy, Clone)]
enum Job<'a> {
    Val(i64),
    Op(Operation<'a>),
}
