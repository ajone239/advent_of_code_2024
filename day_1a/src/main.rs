use std::io;

use anyhow::Result;

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut left = vec![];
    let mut rite = vec![];

    for line in stdin.lines() {
        let line = line.unwrap();

        let mut vals = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());

        left.push(vals.next().unwrap());
        rite.push(vals.next().unwrap());
    }

    left.sort();
    rite.sort();

    let difference: i32 = left
        .into_iter()
        .zip(rite.into_iter())
        .map(|(l, r)| r - l)
        .map(|d| d.abs())
        .sum();

    println!("{}", difference);

    Ok(())
}
