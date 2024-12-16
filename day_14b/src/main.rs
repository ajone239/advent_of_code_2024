use std::{io, str::FromStr};

use anyhow::{Error, Result};

fn main() -> Result<()> {
    let stdin = io::stdin();

    let height = 103;
    let width = 101;

    let mut robots = vec![];

    for line in stdin.lines() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        let robot = Robot::from_str(&line)?;
        robots.push(robot);
    }

    let mut time = 0;
    loop {
        let points = robots.iter().map(|r| r.tick(time, height, width));

        let mut map = vec![vec![' '; width]; height];

        for (x, y) in points {
            map[y][x] = 'x';
        }

        for row in &map {
            let row: String = row.iter().collect();
            println!("{:?}", row);
        }
        println!("{}", time);
        println!();

        time += 1;

        let mut buf = String::new();
        _ = io::stdin().read_line(&mut buf)?;

        let buf = buf.trim();

        if !buf.is_empty() {
            break;
        }
    }

    Ok(())
}

#[derive(Debug)]
struct Robot {
    x: usize,
    y: usize,
    vx: isize,
    vy: isize,
}

impl Robot {
    fn tick(&self, time: usize, height: usize, width: usize) -> (usize, usize) {
        let iheight = height as isize;
        let iwidth = width as isize;

        let vx = ((self.vx + iwidth) % iwidth) as usize;
        let vy = ((self.vy + iheight) % iheight) as usize;

        let dx = time * vx;
        let dy = time * vy;

        let new_x = (self.x + dx) % width;
        let new_y = (self.y + dy) % height;

        (new_x, new_y)
    }
}

impl FromStr for Robot {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let point_str = &parts.next().unwrap()[2..];
        let velocity_str = &parts.next().unwrap()[2..];

        let mut point_coords = point_str.split(',').map(|p| p.parse::<usize>().unwrap());
        let mut velocity_coords = velocity_str.split(',').map(|p| p.parse::<isize>().unwrap());

        let x = point_coords.next().unwrap();
        let y = point_coords.next().unwrap();
        let vx = velocity_coords.next().unwrap();
        let vy = velocity_coords.next().unwrap();

        Ok(Self { x, y, vx, vy })
    }
}
