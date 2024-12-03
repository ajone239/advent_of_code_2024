use anyhow::Result;
use std::io;

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut reports = vec![];

    for line in stdin.lines() {
        let line = line.unwrap();

        let report: Vec<_> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        reports.push(report);
    }

    let count = reports
        .into_iter()
        .map(|report| {
            report
                .windows(2)
                .map(|arr| arr[0] - arr[1])
                .collect::<Vec<_>>()
        })
        // check for monotonic increase or decrease
        .filter(|diffs| diffs.iter().all(|v| v > &0) || diffs.iter().all(|v| v < &0))
        // no change greater than 3
        .filter(|diffs| diffs.iter().map(|v| v.abs()).max().unwrap() <= 3)
        // no change smaller than 1
        .filter(|diffs| diffs.iter().map(|v| v.abs()).min().unwrap() >= 1)
        .count();

    println!("{}", count);

    Ok(())
}
