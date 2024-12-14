use std::{io, str::FromStr};

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut machine_input: Vec<String> = vec![];
    let mut machines: Vec<Game> = vec![];

    // parse
    for line in stdin.lines() {
        let line = line?;

        if line.is_empty() {
            machines.push(Game::from_data(machine_input.clone())?);
            machine_input.clear();
            continue;
        }

        machine_input.push(line)
    }
    machines.push(Game::from_data(machine_input)?);

    println!("{:#?}", machines);

    Ok(())
}

#[derive(Debug)]
struct Button {
    name: char,
    dx: i32,
    dy: i32,
}

impl FromStr for Button {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.split_whitespace().collect_vec();

        let name = data[1].chars().next().unwrap();

        let dx: String = data[2].chars().skip(2).take_while(|c| *c != ',').collect();
        let dx = dx.parse()?;

        let dy: String = data[3].chars().skip(2).take_while(|c| *c != ',').collect();
        let dy = dy.parse()?;

        Ok(Self { name, dx, dy })
    }
}

#[derive(Debug)]
struct Prize {
    x: u32,
    y: u32,
}

impl FromStr for Prize {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.split_whitespace().collect_vec();

        let x: String = data[1].chars().skip(2).take_while(|c| *c != ',').collect();
        let x = x.parse()?;

        let y: String = data[1].chars().skip(2).take_while(|c| *c != ',').collect();
        let y = y.parse()?;

        Ok(Self { x, y })
    }
}

#[derive(Debug)]
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
