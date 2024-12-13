use std::{collections::HashMap, io, rc::Rc, sync::Mutex};

use anyhow::{Ok, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut field: Vec<Vec<char>> = vec![];

    // parse
    for line in stdin.lines() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        field.push(line.trim().chars().collect());
    }

    let mut all_plots: Vec<Rc<Mutex<Plot>>> = vec![];
    let mut rev_index: HashMap<(char, usize, usize), Rc<Mutex<Plot>>> = HashMap::new();

    // get all plots
    let mut id = 0;
    for (i, row) in field.iter().enumerate() {
        for (j, crop) in row.iter().enumerate() {
            let neighbors = get_neighbors(&field, i, j);
            let plots = get_potential_plots(&rev_index, i, j, *crop);

            let plot = match plots.len() {
                0 => {
                    // Add a plot as there is none
                    id += 1;
                    let plot = Plot {
                        id,
                        crop: *crop,
                        tiles: vec![],
                    };
                    let plot = Rc::new(Mutex::new(plot));

                    all_plots.push(plot.clone());

                    plot
                }
                1 => plots.first().unwrap().clone(),
                _ => {
                    merge_plots(plots, &mut all_plots, &mut rev_index)
                }
            };

            // fix reverse index
            rev_index.insert((*crop, i, j), plot.clone());

            let tile = Tile {
                i,
                j,
                crop: *crop,
                neighbors,
            };

            // store the tile
            plot.lock().unwrap().tiles.push(tile);
        }
    }

    let mut result = 0;
    for p in all_plots {
        let p = p.lock().unwrap();
        let area = p.area();
        let perimeter = p.perimeter();

        result += area * perimeter;

        println!("{}: {} {}", p.crop, area, perimeter);
    }

    println!("{}", result);

    Ok(())
}

fn get_neighbors(field: &Vec<Vec<char>>, i: usize, j: usize) -> [Option<char>; 4] {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let neighbors = directions
        .into_iter()
        .map(|(dx, dy)| ((i as isize + dx) as usize, (j as isize + dy) as usize))
        .map(|(i, j)| {
            if i >= field.len() || j >= field[0].len() {
                None
            } else {
                Some(field[i][j])
            }
        });

    let mut rv = [None, None, None, None];

    for (i, n) in neighbors.enumerate() {
        rv[i] = n;
    }

    rv
}

fn get_potential_plots(
    rev_index: &HashMap<(char, usize, usize), Rc<Mutex<Plot>>>,
    i: usize,
    j: usize,
    crop: char,
) -> Vec<Rc<Mutex<Plot>>> {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    directions
        .into_iter()
        .map(|(dx, dy)| ((i as isize + dx) as usize, (j as isize + dy) as usize))
        .map(|(i, j)| rev_index.get(&(crop, i, j)))
        .filter(|n| n.is_some())
        .map(|n| n.unwrap())
        .map(|n| n.clone())
        .filter(|n| n.lock().unwrap().crop == crop)
        .unique_by(|n| n.lock().unwrap().id)
        .collect()
}

fn merge_plots(
    plots_to_merge: Vec<Rc<Mutex<Plot>>>,
    all_plots: &mut Vec<Rc<Mutex<Plot>>>,
    rev_index: &mut HashMap<(char, usize, usize), Rc<Mutex<Plot>>>,
) -> Rc<Mutex<Plot>> {
    let main = plots_to_merge[0].clone();

    let to_be_removed = &plots_to_merge[1..];

    // merge
    for tbr in to_be_removed {
        let mut tbr = tbr.lock().unwrap();

        for tile in &tbr.tiles {
            let crop = tile.crop;
            let i = tile.i;
            let j = tile.j;

            rev_index.entry((crop, i, j)).and_modify(|p| {
                *p = main.clone();
            });

        }
        main.lock().unwrap().tiles.append(&mut tbr.tiles)
    }

    // remove olds
    for tbr in to_be_removed {
        let id_tbr = tbr.lock().unwrap().id;
        let ap_len = all_plots.len();
        for i in (0..ap_len).rev() {
            if all_plots[i].lock().unwrap().id != id_tbr {
                continue;
            }

            all_plots.remove(i);
        }
    }

    main
}

#[derive(Debug)]
struct Tile {
    i: usize,
    j: usize,
    crop: char,
    neighbors: [Option<char>; 4],
}

#[derive(Debug)]
struct Plot {
    id: usize,
    crop: char,
    tiles: Vec<Tile>,
}

impl Plot {
    fn area(&self) -> usize {
        self.tiles.len() as usize
    }

    fn perimeter(&self) -> usize {
        self.tiles
            .iter()
            .map(|t| t.neighbors.iter().filter(|n| **n != Some(t.crop)).count())
            .sum()
    }
}
