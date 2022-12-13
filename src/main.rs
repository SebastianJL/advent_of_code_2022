use std::{error::Error, str::FromStr, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let input = read();
    let lists_str = parse(&input);

    let mut lists: Vec<List<u32>> = lists_str
        .into_iter()
        .map(|list| -> Result<_, String> {
            let list: List<u32> = list.parse()?;
            Ok(list)
        })
        .collect::<Result<_, _>>()?;

    lists.sort();
    let divider1: List<u32> = "[[2]]".parse()?;
    let divider2: List<u32> = "[[6]]".parse()?;

    let i1 = lists.binary_search(&divider1).unwrap() + 1;
    let i2 = lists.binary_search(&divider2).unwrap() + 1;

    dbg!(i1 * i2);

    let runtime = start.elapsed();
    dbg!(runtime);
    Ok(())
}

fn read() -> String {
    std::fs::read_to_string("input.txt").expect("File not found.")
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().filter(|s| !s.is_empty()).collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item<T: Clone> {
    Val(T),
    List(List<T>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct List<T: Clone> {
    items: Vec<Item<T>>,
}

impl<T: Clone> List<T> {
    fn new() -> Self {
        List { items: vec![] }
    }

    fn push(&mut self, item: Item<T>) {
        self.items.push(item);
    }
}

impl FromStr for List<u32> {
    type Err = String;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('[') {
            assert!(s.chars().last().unwrap() == ']');
            s = &s[1..s.len() - 1];
        }
        // dbg!(s);
        let mut list: List<u32> = List::new();
        let mut iter = s.chars().enumerate().peekable();
        while let Some((i0, c)) = iter.next() {
            match c {
                '[' => {
                    // println!("{:?}", &s[i0..s.len()]);
                    let di = find_matching_paren(&s[i0..s.len()]);
                    let i1 = i0 + di;
                    // println!("{i0}, {i1}");
                    // println!("{:?}", &s[i0..=i1]);
                    let sublist: List<u32> = s[i0..=i1].parse()?;
                    // println!("{:?}", sublist);
                    list.push(Item::List(sublist));
                    iter.nth(i1 - i0 - 1).expect("Couldn't advance iterator.");
                }
                ',' => {}
                c if c.is_numeric() => {
                    let mut num = String::from(c);
                    while let Some(&(_, new_c)) = iter.peek() {
                        if !new_c.is_numeric() {
                            break;
                        } else {
                            num.push(iter.next().unwrap().1);
                        }
                    }
                    let item: u32 = num.parse().unwrap();
                    list.push(Item::Val(item));
                }
                c => Err(format!("unkown char {}", c))?,
            }
        }
        Ok(list)
    }
}

impl<T: std::fmt::Display + Clone> std::fmt::Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
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

impl<T: PartialOrd + Clone> PartialOrd for Item<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Item::Val(s), Item::Val(o)) => s.partial_cmp(o),
            (s @ Item::Val(_), Item::List(o)) => {
                let mut l = List::new();
                l.push(s.clone());
                l.partial_cmp(o)
            }
            (Item::List(s), o @ Item::Val(_)) => {
                let mut l = List::new();
                l.push(o.clone());
                s.partial_cmp(&l)
            }
            (Item::List(s), Item::List(o)) => s.partial_cmp(o),
        }
    }
}

impl<T: Ord + Clone> Ord for Item<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
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
