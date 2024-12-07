use std::io;

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut calibrations: Vec<Calibration> = vec![];

    for line in stdin.lines() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        let mut liter = line.split(':');

        let target: i64 = liter.next().unwrap().parse()?;

        let elements: Vec<i64> = liter
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        let cal = Calibration { target, elements };

        calibrations.push(cal);
    }

    let result: i64 = calibrations
        .into_iter()
        .filter(|c| c.is_valid())
        .map(|c| c.target)
        .sum();

    println!("{}", result);

    Ok(())
}

#[derive(Debug)]
struct Calibration {
    target: i64,
    elements: Vec<i64>,
}

impl Calibration {
    fn is_valid(&self) -> bool {
        let upperlimit = Self::all_ones(self.elements.len() - 1);

        for mask in 0..=upperlimit {
            let mut total: i64 = self.elements[0];

            for (i, elem) in self.elements[1..].iter().enumerate() {
                total = if mask & 1 << i > 0 {
                    total * elem
                } else {
                    total + elem
                }
            }

            if total == self.target {
                return true;
            }
        }

        false
    }

    fn all_ones(len: usize) -> u64 {
        let mut rv = 0;
        for i in 0..len {
            rv |= 1 << i;
        }
        rv
    }
}
