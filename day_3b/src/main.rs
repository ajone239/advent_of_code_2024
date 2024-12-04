use std::io;

use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let stdin = io::stdin();

    let input: String = stdin.lines().map(|l| l.unwrap()).collect();

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    let matches = re.captures_iter(&input);

    let mut commands = vec![];
    for m in matches {
        let left: u32 = m[1].parse()?;
        let right: u32 = m[2].parse()?;

        let command = MulCommand { left, right };
        commands.push(command);
    }

    let result: i32 = commands.iter().map(|c| c.evaluate()).sum();

    println!("{}", result);

    Ok(())
}

trait Evaluate {
    fn evaluate(&self) -> i32;
}

#[derive(Debug)]
struct MulCommand {
    left: u32,
    right: u32,
}

impl Evaluate for MulCommand {
    fn evaluate(&self) -> i32 {
        (self.left * self.right) as i32
    }
}
