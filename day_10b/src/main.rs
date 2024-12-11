use std::{collections::HashSet, io};

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut map: Vec<Vec<u8>> = vec![];

    for line in stdin.lines() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        let line = line
            .chars()
            .map(|c| match c {
                '.' => u8::MAX,
                c => c as u8 - '0' as u8,
            })
            .collect();
        map.push(line);
    }

    let starts: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, cell)| **cell == 0)
                .map(move |(j, _)| (i, j))
        })
        .collect();

    let result: u32 = starts
        .into_iter()
        .map(|s| {
            let mut summits = HashSet::new();
            let score = score_trail_head(s, &map, &mut summits);

            // summits.len() as u32
            score
        })
        .sum();

    println!("{:?}", result);

    Ok(())
}

fn score_trail_head(
    cell: (usize, usize),
    map: &Vec<Vec<u8>>,
    summits: &mut HashSet<(usize, usize)>,
) -> u32 {
    let (i, j) = cell;
    let cell_value = map[i][j];

    if cell_value == 9 {
        summits.insert(cell);
        return 1;
    }

    let directions = [(0, 1), (1, 0), (-1, 0), (0, -1)];

    let mut sum = 0;

    for dir in directions {
        let new_i = (i as isize + dir.0) as usize;
        let new_j = (j as isize + dir.1) as usize;

        if new_i >= map.len() || new_j >= map[0].len() {
            continue;
        }

        let new_cell_value = map[new_i][new_j];

        if new_cell_value != cell_value + 1 {
            continue;
        }

        sum += score_trail_head((new_i, new_j), map, summits);
    }

    sum
}
