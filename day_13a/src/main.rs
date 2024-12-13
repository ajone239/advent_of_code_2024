use std::{io, str::FromStr};

use anyhow::Result;

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut machine_input: Vec<String> = vec![];
    let mut machines: Vec<Vec<String>> = vec![];

    // parse
    for line in stdin.lines() {
        let line = line?;

        if line.is_empty() {
            machines.push(machine_input.clone());
            machine_input.clear();
            continue;
        }

        machine_input.push(line)
    }
    machines.push(machine_input.clone());
    machine_input.clear();

    println!("{:#?}", machines);

    Ok(())
}

struct Button {
    name: char,
    dx: i32,
    dy: i32,
}

impl FromStr for Button {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

struct Prize {
    x: u32,
    y: u32,
}

impl FromStr for Prize {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

struct Game {
    a_button: Button,
    b_button: Button,
    prize: Prize,
}

impl Game {
    fn from_data(data: Vec<String>) -> Result<Self> {
        let a_button = Button::from_str(&data[0])?;
        let b_button = Button::from_str(&data[1])?;
        let prize = Prize::from_str(&data[2])?;

        Ok(Self {
            a_button,
            b_button,
            prize,
        })
    }
}
