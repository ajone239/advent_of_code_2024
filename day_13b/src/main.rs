use std::{io, str::FromStr};

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut machine_input: Vec<String> = vec![];
    let mut machines: Vec<Game> = vec![];

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

    for machine in machines {
        println!("{:?}", machine);
        println!("{:?}", machine.min_tokens_to_win());
    }

    Ok(())
}

#[derive(Debug)]
struct Button {
    _name: char,
    dx: i64,
    dy: i64,
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

        Ok(Self {
            _name: name,
            dx,
            dy,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Prize {
    x: i64,
    y: i64,
}

impl FromStr for Prize {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.split_whitespace().collect_vec();

        let x: String = data[1].chars().skip(2).take_while(|c| *c != ',').collect();
        let x: i64 = x.parse()?;

        let y: String = data[2].chars().skip(2).take_while(|c| *c != ',').collect();
        let y: i64 = y.parse()?;

        Ok(Self {
            x: x + 10000000000000,
            y: y + 10000000000000,
        })
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

    fn min_tokens_to_win(&self) -> Option<i64> {
        let mut tokens = None;

        for i in 0..20000 {
            let mut attempt_tokens = 0;
            let mut attempt_at_prize = Prize { x: 0, y: 0 };

            for j in 0..(i + 10000) {
                let (cost, dx, dy) = if j >= i {
                    (1, self.b_button.dx, self.b_button.dy)
                } else {
                    (3, self.a_button.dx, self.a_button.dy)
                };

                attempt_tokens += cost;
                attempt_at_prize.x += dx;
                attempt_at_prize.y += dy;

                if attempt_at_prize > self.prize {
                    break;
                }

                if attempt_at_prize == self.prize {
                    let t = tokens.take().unwrap_or(i64::MAX);
                    tokens = Some(i64::min(t, attempt_tokens));
                    break;
                }
            }
        }

        tokens
    }
}
