use std::io;

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut lines: Vec<String> = vec![];

    for line in stdin.lines() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        lines.push(line);
    }

    for line in lines {
        println!("{}", line);
    }

    Ok(())
}
