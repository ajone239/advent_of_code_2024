use std::{collections::HashMap, io};

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut field: Vec<Vec<char>> = vec![];

    for line in stdin.lines() {
        let line = line?;

        field.push(line.trim().chars().collect());
    }

    let mut rev_index: HashMap<(usize, usize), Plot> = HashMap::new();

    for (i, row) in field.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let mut tile = Tile {
                i,
                j,
                crop: *cell,
                neighbors: [None; 4],
            };
        }
    }

    Ok(())
}

struct Tile {
    i: usize,
    j: usize,
    crop: char,
    neighbors: [Option<char>; 4],
}

struct Plot {
    tiles: Vec<Tile>,
}
