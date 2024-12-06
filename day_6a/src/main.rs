use std::{
    collections::HashSet,
    fmt::{Debug, Write},
    io,
};

use anyhow::Result;

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut lines: Vec<Vec<char>> = vec![];

    for line in stdin.lines() {
        let line = line?;

        lines.push(line.chars().collect());
    }

    let mut map = Map::new(lines);

    println!("{:?}", map);
    println!();

    map.advance();
    map.advance();
    map.advance();
    map.advance();
    map.advance();
    map.advance();
    map.advance();

    println!("{:?}", map);
    println!();


    Ok(())
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum PathState {
    New,
    Looping,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate_cw(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn to_delta(&self) -> (isize, isize) {
        match self {
            Self::North => (-1, 0),
            Self::East => (0, 1),
            Self::South => (1, 0),
            Self::West => (0, -1),
        }
    }
}

#[derive(Copy, Clone)]
enum Square {
    Unvisited,
    Visited,
    Wall,
    Guard,
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let disp = match self {
            Self::Unvisited => '.',
            Self::Visited => '+',
            Self::Wall => '#',
            Self::Guard => '@',
        };

        f.write_char(disp)
    }
}

struct Map {
    width: usize,
    height: usize,
    map: Vec<Vec<Square>>,
    guard_direction: Direction,
    guard_location: (usize, usize),
    seen: HashSet<(usize, usize, Direction)>,
}

impl Map {
    fn new(map: Vec<Vec<char>>) -> Self {
        let width = map[0].len();
        let height = map.len();
        let mut processed_map = vec![vec![Square::Unvisited; map[0].len()]; map.len()];
        let mut seen = HashSet::new();
        let mut guard_direction: Direction = Direction::North;
        let mut guard_location: (usize, usize) = (0, 0);

        for (i, row) in map.into_iter().enumerate() {
            for (j, cell) in row.into_iter().enumerate() {
                match cell {
                    '#' => processed_map[i][j] = Square::Wall,
                    '^' | '<' | '>' | 'v' => processed_map[i][j] = Square::Guard,
                    _ => continue,
                }

                let direction = match cell {
                    '^' => Direction::North,
                    '<' => Direction::West,
                    '>' => Direction::East,
                    'v' => Direction::South,
                    _ => continue,
                };

                guard_direction = direction;
                guard_location = (i, j);
            }
        }

        Self {
            width,
            height,
            map: processed_map,
            guard_location,
            guard_direction,
            seen,
        }
    }

    fn advance(&mut self) -> PathState {
        self.seen.insert((
            self.guard_location.0,
            self.guard_location.1,
            self.guard_direction,
        ));

        let (mut dx, mut dy) = self.guard_direction.to_delta();

        let (old_x, old_y) = self.guard_location;

        let mut new_x = (self.guard_location.0 as isize + dx) as usize;
        let mut new_y = (self.guard_location.1 as isize + dy) as usize;

        while new_x >= self.height || new_y >= self.width {
            self.guard_direction = self.guard_direction.rotate_cw();

            (dx, dy) = self.guard_direction.to_delta();

            new_x = (self.guard_location.0 as isize + dx) as usize;
            new_y = (self.guard_location.1 as isize + dy) as usize;
        }

        let new_tile = (new_x, new_y, self.guard_direction);

        if self.seen.contains(&new_tile) {
            return PathState::Looping;
        }


        self.map[old_x][old_y] = Square::Visited;
        self.map[new_x][new_y] = Square::Guard;

        self.guard_location = (new_x, new_y);

        PathState::New
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.guard_direction)?;
        writeln!(f, "{:?}", self.guard_location)?;
        writeln!(f, "{:?}", self.seen)?;

        for row in &self.map {
            writeln!(f, "{:?}", row)?;
        }

        Ok(())
    }
}
