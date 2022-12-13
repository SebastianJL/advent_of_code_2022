#![feature(iter_advance_by)]
use std::{error::Error, str::FromStr, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let input = read();
    let lists = parse(&input)?;

    for (list1, list2) in lists.iter().copied() {
        let (list1, list2): (List, List) = (list1.parse()?, list2.parse()?);
        println!("{:}\n{:}\n", list1, list2);
    }

    let runtime = start.elapsed();
    dbg!(runtime);
    Ok(())
}

fn read() -> String {
    std::fs::read_to_string("input.txt").expect("File not found.")
}

fn parse(input: &str) -> Result<Vec<(&str, &str)>, String> {
    input
        .split("\n\n")
        .map(|s| {
            let Some((left, right)) = s.split_once('\n') else {
            Err("Input parse error")?
        };
            Ok((left, right))
        })
        .collect::<Result<_, _>>()
}

#[derive(Debug)]
enum Item {
    Val(u32),
    List(List),
}

#[derive(Debug)]
struct List {
    items: Vec<Item>,
}

impl List {
    fn new(items: Vec<Item>) -> Self {
        List { items }
    }

    fn push(&mut self, item: Item) {
        self.items.push(item);
    }
}

impl FromStr for List {
    type Err = String;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('[') {
            assert!(s.chars().last().unwrap() == ']');
            s = &s[1..s.len() - 1];
        }
        // dbg!(s);
        let mut list: List = List::new(vec![]);
        let mut iter = s.chars().enumerate();
        while let Some((i0, c)) = iter.next() {
            match c {
                '[' => {
                    // println!("{:?}", &s[i0..s.len()]);
                    let di = find_matching_paren(&s[i0..s.len()]);
                    let i1 = i0 + di;
                    // println!("{i0}, {i1}");
                    // println!("{:?}", &s[i0..=i1]);
                    let sublist: List = s[i0..=i1].parse()?;
                    // println!("{:?}", sublist);
                    list.push(Item::List(sublist));
                    iter.advance_by(i1 - i0)
                        .expect("Couldn't advance iterator.");
                }
                ',' => {}
                c if c.is_numeric() => {
                    let item: u32 = c.to_digit(10).unwrap();
                    list.push(Item::Val(item));
                }
                c => Err(format!("unkown char {}", c))?,
            }
        }
        Ok(list)
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[");
        let len = self.items.len();
        if len > 0 {
            for item in self.items.iter().take(len - 1) {
                match item {
                    Item::Val(v) => write!(f, "{v},")?,
                    Item::List(l) => {
                        l.fmt(f)?;
                        write!(f, ",")?
                    }
                };
            }
        }

        if let Some(item) = self.items.last() {
            match item {
                Item::Val(v) => write!(f, "{v}]")?,
                Item::List(l) => {
                    l.fmt(f)?;
                    write!(f, "]")?
                }
            };
        } else {
            write!(f, "]")?;
        }

        Ok(())
    }
}

fn find_matching_paren(s: &str) -> usize {
    assert!(s.starts_with('['));
    let mut stack = 0;
    let mut index = s.len() - 1;
    for (i, c) in s.chars().enumerate() {
        match c {
            '[' => stack += 1,
            ']' => {
                stack -= 1;
                if stack == 0 {
                    index = i;
                    break;
                }
            }
            _ => {}
        }
    }
    index
}

