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

    let mut safe_count = 0;
    for mut report in reports.into_iter() {
        let errors = find_error_events(&report);

        if 0 == errors.len() {
            safe_count += 1;
            continue;
        }

        let event_to_remove = errors[0];

        let val = report.remove(event_to_remove);

        let errors = find_error_events(&report);

        if errors.is_empty() {
            safe_count += 1;
            continue;
        }

        report.insert(event_to_remove, val);

        report.remove(event_to_remove + 1);

        let errors = find_error_events(&report);

        if errors.is_empty() {
            safe_count += 1;
            continue;
        }
    }

    println!("{}", safe_count);

    Ok(())
}

fn find_error_events(report: &Vec<i32>) -> Vec<usize> {
    let diffs: Vec<(usize, i32)> = report
        .windows(2)
        .enumerate()
        .map(|(i, arr)| (i, arr[0] - arr[1]))
        .collect();

    let pos_count = diffs.iter().filter(|(_, v)| *v > 0).count();
    let neg_count = diffs.iter().filter(|(_, v)| *v < 0).count();

    let pon = pos_count > neg_count;

    diffs
        .into_iter()
        .filter(|(_, diff)| {
            let val = diff.abs();
            val < 1 || val > 3 || (*diff > 0) != pon
        })
        .map(|(i, _)| i)
        .collect()
}
