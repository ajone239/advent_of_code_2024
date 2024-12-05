use std::{borrow::BorrowMut, io};

use anyhow::Result;

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut puzzle: Vec<Vec<char>> = vec![];

    for line in stdin.lines() {
        let line = line?;

        let row = line.chars().collect();

        puzzle.push(row);
    }

    /* The shape of the mask made in our checks
     *
     *     ****
     *    ***
     *   * * *
     *  *  *  *
     */
    let directions = [(0, 1), (1, 0), (-1, 1), (1, 1)];

    let cross_word = CrossWord::new(puzzle);

    let mut count = 0;

    for i in 0..cross_word.length {
        for j in 0..cross_word.width {
            for dir in directions {
                let i = i as isize;
                let j = j as isize;

                let word: String = (0..4)
                    .map(|k| (i + k * dir.0, j + k * dir.1))
                    .map(|(i, j)| cross_word.bounded_at(i, j))
                    .filter(|c| c.is_some())
                    .map(|c| c.unwrap())
                    .collect();

                if word == "XMAS" || word == "SAMX" {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);

    Ok(())
}

struct CrossWord {
    length: usize,
    width: usize,
    puzzle: Vec<Vec<char>>,
}

impl CrossWord {
    fn new(puzzle: Vec<Vec<char>>) -> Self {
        let length = puzzle.len();
        let width = puzzle[0].len();

        Self {
            length,
            width,
            puzzle,
        }
    }

    fn bounded_at(&self, i: isize, j: isize) -> Option<char> {
        if i < 0 || j < 0 || i >= self.length as isize || j >= self.width as isize {
            return None;
        }

        let i = i as usize;
        let j = j as usize;

        Some(self.puzzle[i][j])
    }
}
