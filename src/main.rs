use std::{
    error::Error,
    fmt::Display,
    ops::{Index, IndexMut},
    time::Instant,
};

use take_until::TakeUntilExt;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let input = read();
    let data = parse(&input);

    let n_rows = input.lines().count();
    let size = data.len();
    let n_cols = size / n_rows;
    assert_eq!(n_cols * n_rows, size, "Size is not divisible by n_rows.");

    let grid = Array2::new(data, n_rows, n_cols);
    let mut scores = Array2::<usize>::like(&grid);
    for row in 1..(n_rows - 1) {
        for col in 1..(n_cols - 1) {
            let height = grid[[row, col]];
            let see_left = (0..col)
                .rev()
                .take_until(|&j| height <= grid[[row, j]])
                .count();
            let see_right = (col+1..n_cols)
                .take_until(|&j| height <= grid[[row, j]])
                .count();
            let see_up = (0..row)
                .rev()
                .take_until(|&i| height <= grid[[i, col]])
                .count();
            let see_down = (row+1..n_rows)
                .take_until(|&i| height <= grid[[i, col]])
                .count();
            scores[[row, col]] += see_left*see_right*see_up*see_down;
        }
    }


    println!("{}", scores.data.iter().max().unwrap());

    let runtime = start.elapsed();
    dbg!(runtime);
    Ok(())
}

fn read() -> String {
    std::fs::read_to_string("input.txt").expect("File not found.")
}

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
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
