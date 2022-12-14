use std::{collections::HashSet, error::Error, ops::Add, str::FromStr, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let input = read();
    let instructions = parse(&input);
    let mut obstacles = build_walls(&instructions);
    // dbg!(walls);
    let ymax = obstacles.iter().map(|o| o.y).max().unwrap() + 2;
    let mut run = true;
    let mut count = 0;
    let source = Point { x: 500, y: 0 };
    while run {
        let mut sandcorn = source;
        let mut fall = true;
        count += 1;
        while fall {
            if sandcorn.y == ymax  - 1 {
                fall = false;
                obstacles.insert(sandcorn);
            } else if !obstacles.contains(&(sandcorn + (0, 1))) {
                sandcorn = sandcorn + (0, 1);
            } else if !obstacles.contains(&(sandcorn + (-1, 1))) {
                sandcorn = sandcorn + (-1, 1);
            } else if !obstacles.contains(&(sandcorn + (1, 1))) {
                sandcorn = sandcorn + (1, 1);
            } else {
                fall = false;
                obstacles.insert(sandcorn);
                if sandcorn == source {
                    run = false;
                }
            }
        }
    }
    dbg!(count);

    let runtime = start.elapsed();
    dbg!(runtime);
    Ok(())
}

fn build_walls(instructions: &[Vec<Point>]) -> HashSet<Point> {
    let mut walls = HashSet::new();
    for point_vec in instructions {
        for win in point_vec.windows(2) {
            let (p1, p2) = (win[0], win[1]);
            if p1.x == p2.x {
                let (ymin, ymax) = (p1.y.min(p2.y), p1.y.max(p2.y));
                for yi in ymin..=ymax {
                    walls.insert(Point { x: p1.x, y: yi });
                }
            } else if p1.y == p2.y {
                let (xmin, xmax) = (p1.x.min(p2.x), p1.x.max(p2.x));
                for xi in xmin..=xmax {
                    walls.insert(Point { x: xi, y: p1.y });
                }
            } else {
                panic!("malformed instructions")
            }
        }
    }

    walls
}

fn read() -> String {
    std::fs::read_to_string("input.txt").expect("File not found.")
}

fn parse(input: &str) -> Vec<Vec<Point>> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|p_str| p_str.parse::<Point>().unwrap())
                .collect()
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((x, y)) = s.split_once(',') else {
            Err("Couldn't split input.")?
        };
        Ok(Point {
            x: x.parse().or(Err("Couldn't parse left part.".to_owned()))?,
            y: y.parse().or(Err("Couldn't parse right part.".to_owned()))?,
        })
    }
}

impl Add<(isize, isize)> for Point {
    type Output = Self;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Point {
            x: (self.x as isize + rhs.0) as usize,
            y: (self.y as isize + rhs.1) as usize,
        }
    }
}
