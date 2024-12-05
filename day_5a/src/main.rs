use std::io;

use anyhow::Result;

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut orderings: Vec<String> = vec![];
    let mut updates: Vec<String> = vec![];

    let mut oou = true;
    for line in stdin.lines() {
        let line = line?;

        if line.is_empty() {
            oou = false;
            continue;
        }

        if oou {
            orderings.push(line);
        } else {
            updates.push(line)
        }
    }

    let orderings: Vec<(usize, usize)> = orderings
        .into_iter()
        .map(|s| {
            s.split('|')
                .map(|sp| sp.parse::<usize>().unwrap())
                .collect()
        })
        .map(|i: Vec<usize>| (i[0], i[1]))
        .collect();

    let updates: Vec<Vec<usize>> = updates
        .into_iter()
        .map(|s| {
            s.split(',')
                .map(|sp| sp.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    println!("{:?}", orderings);
    println!();
    println!("{:?}", updates);

    Ok(())
}
