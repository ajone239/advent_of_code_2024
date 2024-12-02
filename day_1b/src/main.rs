use std::{collections::HashMap, io};

use anyhow::Result;

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut left = vec![];
    let mut rite = HashMap::new();

    for line in stdin.lines() {
        let line = line.unwrap();

        let mut vals = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());

        left.push(vals.next().unwrap());

        let rite_val = vals.next().unwrap();

        *rite.entry(rite_val).or_insert(0) += 1;
    }

    left.sort();

    let difference: i32 = left
        .into_iter()
        .map(|l| l * rite.get(&l).unwrap_or(&0))
        .sum();

    println!("{}", difference);

    Ok(())
}
