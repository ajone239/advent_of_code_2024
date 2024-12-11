use std::{collections::VecDeque, io};

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let stdin = io::stdin();

    for line in stdin.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}
