use std::{
    collections::{HashMap, HashSet},
    f64, io,
};

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut lines: Vec<Vec<char>> = vec![];

    for line in stdin.lines() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        lines.push(line.chars().collect());
    }

    let width = lines.len() as isize;
    let height = lines[0].len() as isize;

    let mut antenas: HashMap<char, Vec<Point>> = HashMap::new();

    for (i, row) in lines.into_iter().enumerate() {
        for (j, cell) in row.into_iter().enumerate() {
            if cell == '.' {
                continue;
            }

            let point = Point {
                x: i as isize,
                y: j as isize,
            };

            antenas.entry(cell).or_default().push(point);
        }
    }

    let mut antinodes = HashSet::new();

    for (_, towers) in &antenas {
        for i in 0..towers.len() - 1 {
            let start = &towers[i];
            for other in &towers[i + 1..] {
                let (f, b) = start.project(other);

                antinodes.insert(f);
                antinodes.insert(b);
            }
        }
    }

    let count = antinodes
        .into_iter()
        .filter(|Point { x, y }| *x < width && *y < height && *y >= 0 && *x >= 0)
        .count();

    println!("{:?}", count);

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn project(&self, other: &Self) -> (Point, Point) {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;

        let forward = Point {
            x: self.x + x_diff,
            y: self.y + y_diff,
        };
        let backward = Point {
            x: other.x - x_diff,
            y: other.y - y_diff,
        };

        (forward, backward)
    }
}
