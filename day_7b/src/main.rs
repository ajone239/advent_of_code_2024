use core::panic;
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
        let digits = AnyBaseDigits::new(3);

        let power_set_count = 3_i32.pow((self.elements.len() - 1) as u32) as usize;

        for mask in digits.take(power_set_count) {
            let mut total: i64 = self.elements[0];

            for (i, elem) in self.elements[1..].iter().enumerate() {
                total = match mask[i] {
                    0 => total + elem,
                    1 => total * elem,
                    2 => {
                        let mut melem = *elem;
                        let mut elem_tens = 1;
                        while melem > 0 {
                            elem_tens *= 10;
                            melem /= 10;
                        }

                        total * elem_tens + elem
                    }
                    _ => panic!(),
                }
            }

            if total == self.target {
                return true;
            }
        }

        false
    }
}

struct AnyBaseDigits {
    base: usize,
    digits: [usize; 32],
}

impl AnyBaseDigits {
    fn new(base: usize) -> Self {
        let digits = [0; 32];

        Self { base, digits }
    }
}

impl Iterator for AnyBaseDigits {
    type Item = [usize; 32];

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = 0;
        let rv = self.digits.clone();
        while i < self.digits.len() {
            let digit = &mut self.digits[i];
            *digit += 1;

            if *digit < self.base {
                break;
            }

            *digit = 0;
            i += 1;
        }

        Some(rv)
    }
}
