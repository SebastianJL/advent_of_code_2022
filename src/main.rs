use std::{collections::VecDeque, error::Error, str::FromStr, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    // let mut monkeys = vec![
    //     Monkey {
    //         items: vec![79, 98].into(),
    //         op: |old| old * 19,
    //         divisible: |val| if val % 23 == 0 { 2 } else { 3 },
    //     },
    //     Monkey {
    //         items: vec![54, 65, 75, 74].into(),
    //         op: |old| old + 6,
    //         divisible: |val| if val % 19 == 0 { 2 } else { 0 },
    //     },
    //     Monkey {
    //         items: vec![79, 60, 97].into(),
    //         op: |old| old * old,
    //         divisible: |val| if val % 13 == 0 { 1 } else { 3 },
    //     },
    //     Monkey {
    //         items: vec![74].into(),
    //         op: |old| old + 3,
    //         divisible: |val| if val % 17 == 0 { 0 } else { 1 },
    //     },
    // ];

    let mut monkeys = vec![
        Monkey {
            items: vec![89, 73, 66, 57, 64, 80].into(),
            op: |old| old * 3,
            divisible: |val| if val % 13 == 0 { 6 } else { 2 },
        },
        Monkey {
            items: vec![83, 78, 81, 55, 81, 59, 69].into(),
            op: |old| old + 1,
            divisible: |val| if val % 3 == 0 { 7 } else { 4 },
        },
        Monkey {
            items: vec![76, 91, 58, 85].into(),
            op: |old| old * 13,
            divisible: |val| if val % 7 == 0 { 1 } else { 4 },
        },
        Monkey {
            items: vec![71, 72, 74, 76, 68].into(),
            op: |old| old * old,
            divisible: |val| if val % 2 == 0 { 6 } else { 0 },
        },
        Monkey {
            items: vec![98, 85, 84].into(),
            op: |old| old + 7,
            divisible: |val| if val % 19 == 0 { 5 } else { 7 },
        },
        Monkey {
            items: vec![78].into(),
            op: |old| old + 8,
            divisible: |val| if val % 5 == 0 { 3 } else { 0 },
        },
        Monkey {
            items: vec![86, 70, 60, 88, 88, 78, 74, 83].into(),
            op: |old| old + 4,
            divisible: |val| if val % 11 == 0 { 1 } else { 2 },
        },
        Monkey {
            items: vec![81, 58].into(),
            op: |old| old + 5,
            divisible: |val| if val % 17 == 0 { 3 } else { 5 },
        },
    ];

    let n_rounds = 20;
    let mut inspected = vec![0; monkeys.len()];
    for _ in 0..n_rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let n_inspected = monkey.inspect();
            inspected[i] += n_inspected;
            monkey.apply(|item| item / 3);
            let throw_instructions = monkey.throw();
            for (dest, item) in throw_instructions {
                monkeys[dest].items.push_back(item);
            }
        }
    }

    println!("{monkeys:?}");
    inspected.sort_by(|a, b| b.cmp(a));
    let total = inspected[0] * inspected[1];
    dbg!(total);

    let runtime = start.elapsed();
    dbg!(runtime);
    Ok(())
}

fn read() -> String {
    std::fs::read_to_string("input.txt").expect("File not found.")
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u32>,
    op: fn(u32) -> u32,
    divisible: fn(u32) -> usize,
}

impl Monkey {
    fn inspect(&mut self) -> usize {
        let n_inspected = self.items.len();
        for item in self.items.iter_mut() {
            *item = (self.op)(*item);
        }
        n_inspected
    }

    fn apply(&mut self, f: fn(u32) -> u32) {
        for item in self.items.iter_mut() {
            *item = (f)(*item);
        }
    }

    fn throw(&mut self) -> Vec<(usize, u32)> {
        let items = std::mem::replace(&mut self.items, vec![].into());
        items
            .into_iter()
            .map(|item| ((self.divisible)(item), item))
            .collect()
    }
}
