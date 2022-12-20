use std::{collections::HashSet, error::Error, time::Instant};

use ndarray::Array2;

const START: i32 = -1;
const END: i32 = 26;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let input = read();
    let map = parse(&input);
    dbg!(&map);

    let shortest_path = flow(&map);

    dbg!(shortest_path);

    let runtime = start.elapsed();
    dbg!(runtime);
    Ok(())
}

fn find_index(element: i32, map: &Array2<i32>) -> Option<(usize, usize)> {
    map.outer_iter().enumerate().find_map(|(i, row)| {
        match row.into_iter().position(|&x| x == element) {
            Some(j) => Some((i, j)),
            None => None,
        }
    })
}

fn flow(map: &Array2<i32>) -> u32 {
    let start_idx = find_index(START, &map).unwrap();
    let end_idx = find_index(END, &map).unwrap();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut rim = HashSet::new();
    let mut new_rim = HashSet::new();

    visited.insert(start_idx);
    rim.insert(start_idx);

    let mut count = 0;
    loop {
        count += 1;

        new_rim.clear();
        for &idx in rim.iter() {
            // Check up
            if idx.0 > 0 {
                let up = (idx.0 - 1, idx.1);
                if map[up] - map[idx] <= 1 && !visited.contains(&up) {
                    new_rim.insert(up);
                }
            }

            // Check down
            if idx.0 < map.dim().0 - 1 {
                let down = (idx.0 + 1, idx.1);
                if map[down] - map[idx] <= 1 && !visited.contains(&down) {
                    new_rim.insert(down);
                }
            }

            // Check left
            if idx.1 > 0 {
                let left = (idx.0, idx.1 - 1);
                if map[left] - map[idx] <= 1 && !visited.contains(&left) {
                    new_rim.insert(left);
                }
            }

            // Check right
            if idx.1 < map.dim().1 - 1 {
                let right = (idx.0, idx.1 + 1);
                if map[right] - map[idx] <= 1 && !visited.contains(&right) {
                    new_rim.insert(right);
                }
            }
        }
        if new_rim.contains(&end_idx) {
            break;
        }
        visited.extend(&new_rim);
        std::mem::swap(&mut rim, &mut new_rim);
    }

    count
}

fn read() -> String {
    std::fs::read_to_string("input.txt").expect("File not found.")
}

fn parse(input: &str) -> Array2<i32> {
    let n_cols = input.lines().next().unwrap().chars().count();
    let n_rows = input.lines().count();

    let map: Vec<i32> = input
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                'S' => START,
                'E' => END,
                c => c as i32 - 'a' as i32,
            })
        })
        .collect();

    let map = Array2::from_shape_vec([n_rows, n_cols], map).unwrap();
    map
}
