use std::{
    error::Error,
    fmt::Display,
    ops::{Index, IndexMut},
    time::Instant,
    vec,
};

use itertools::izip;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let input = read();
    let data = parse(&input);

    let n_rows = input.lines().count();
    let size = data.len();
    let n_cols = size / n_rows;
    assert_eq!(n_cols * n_rows, size, "Size is not divisible by n_rows.");

    let grid = Array2::new(data, n_rows, n_cols);

    let sees_left = look_left(&grid);
    let sees_right = look_right(&grid);
    let sees_up = look_up(&grid);
    let sees_down = look_down(&grid);
    println!("{sees_left}");
    println!("{sees_right}");
    println!("{sees_up}");
    println!("{sees_down}");

    let total: i32 = izip!(
        sees_left.data,
        sees_right.data,
        sees_up.data,
        sees_down.data
    )
    .map(|(one, two, three, four)| (one || two || three || four) as i32)
    .sum();

    dbg!(total);

    let runtime = start.elapsed();
    dbg!(runtime);
    Ok(())
}

fn look_left(grid: &Array2<i32>) -> Array2<bool> {
    let mut sees_left = Array2::<bool>::like(grid);

    for i in 0..grid.n_rows {
        let mut max = -1;
        for j in 0..grid.n_cols {
            sees_left[[i, j]] = grid[[i, j]] > max;
            max = max.max(grid[[i, j]]);
        }
    }

    sees_left
}

fn look_right(grid: &Array2<i32>) -> Array2<bool> {
    let mut sees_right = Array2::<bool>::like(grid);

    for i in 0..grid.n_rows {
        let mut max = -1;
        for j in (0..grid.n_cols).rev() {
            sees_right[[i, j]] = grid[[i, j]] > max;
            // dbg!((grid[[i, j]], max));
            max = max.max(grid[[i, j]]);
        }
    }

    sees_right
}

fn look_up(grid: &Array2<i32>) -> Array2<bool> {
    let mut sees_up = Array2::<bool>::like(grid);

    for j in 0..grid.n_cols {
        let mut max = -1;
        for i in 0..grid.n_rows {
            sees_up[[i, j]] = grid[[i, j]] > max;
            max = max.max(grid[[i, j]]);
        }
    }

    sees_up
}

fn look_down(grid: &Array2<i32>) -> Array2<bool> {
    let mut sees_down = Array2::<bool>::like(grid);

    for j in 0..grid.n_cols {
        let mut max = -1;
        for i in (0..grid.n_rows).rev() {
            sees_down[[i, j]] = grid[[i, j]] > max;
            max = max.max(grid[[i, j]]);
        }
    }

    sees_down
}

fn read() -> String {
    std::fs::read_to_string("input.txt").expect("File not found.")
}

fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .flatten()
        .collect()
}

#[derive(Debug)]
struct Array2<T: Copy> {
    n_rows: usize,
    n_cols: usize,
    data: Vec<T>,
}

impl<T: Copy + Default> Array2<T> {
    fn new(data: Vec<T>, n_rows: usize, n_cols: usize) -> Self {
        assert_eq!(n_rows * n_cols, data.len());
        Array2 {
            data,
            n_rows,
            n_cols,
        }
    }

    fn like<U: Copy + Default>(similar: &Array2<U>) -> Array2<T> {
        let size = similar.n_rows * similar.n_cols;
        let data: Vec<T> = vec![Default::default(); size];

        Array2 {
            data,
            n_rows: similar.n_rows,
            n_cols: similar.n_cols,
        }
    }
}

impl<T: Copy> Index<[usize; 2]> for Array2<T> {
    type Output = T;

    fn index(&self, [i, j]: [usize; 2]) -> &Self::Output {
        &self.data[i * self.n_cols + j]
    }
}

impl<T: Copy> IndexMut<[usize; 2]> for Array2<T> {
    fn index_mut(&mut self, [i, j]: [usize; 2]) -> &mut Self::Output {
        &mut self.data[i * self.n_cols + j]
    }
}

impl<T: Copy + std::fmt::Debug> Display for Array2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.n_rows {
            let start = row * self.n_cols;
            let end = start + self.n_cols;
            writeln!(f, "{:?}", &self.data[start..end])?;
        }
        Ok(())
    }
}

// impl<T: Copy> IntoIterator for Array2<T> {
//     type Item = T;

//     type IntoIter;

//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }

// struct Array2Iter {

// }

// impl Iterator for
