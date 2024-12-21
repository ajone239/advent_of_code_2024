use std::io;

use anyhow::Result;

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut map: Vec<Vec<char>> = vec![];
    let mut moves: Vec<char> = vec![];

    let mut mom = true;
    for line in stdin.lines() {
        let line = line?;

        if line.is_empty() {
            mom = false;
            continue;
        }

        if mom {
            map.push(line.chars().collect());
        } else {
            moves.extend(line.chars());
        }
    }

    let mut map: Vec<Vec<Tile>> = map
        .into_iter()
        .map(|row| row.into_iter().map(Tile::from_char).collect())
        .collect();

    let moves: Vec<Direction> = moves.into_iter().map(Direction::from_char).collect();

    for row in map {
        println!("{:?}", row);
    }

    println!("{:?}", moves);

    Ok(())
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
enum Tile {
    Empty,
    Wall,
    Robot,
    Box,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            'O' => Tile::Box,
            '@' => Tile::Robot,
            _ => panic!(),
        }
    }
}
