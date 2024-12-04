use std::io;

use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let stdin = io::stdin();

    let input: String = stdin.lines().map(|l| l.unwrap()).collect();

    let re = Regex::new(r"(do|don't|mul)\(((\d{1,3}),(\d{1,3}))?\)")?;

    let matches = re.captures_iter(&input);

    let mut enable = true;
    let mut commands = vec![];

    for m in matches {
        match &m[1] {
            "do" => {
                enable = true;
                continue;
            }
            "don't" => {
                enable = false;
                continue;
            }
            _ => (),
        }

        if !enable {
            continue;
        }

        let left: u32 = m[3].parse()?;
        let right: u32 = m[4].parse()?;

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
