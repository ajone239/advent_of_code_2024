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

        if line.is_empty() {
            break;
        }

        lines.push(line.chars().collect());
    }

    let mut map = Map::new(lines);

    while PathState::New == map.advance() {}

    let mut count = 0;
    for (x, y, dir) in map.seen.iter() {
        if map.can_loop(*dir, *x, *y) {
            count += 1;
            println!("cool {count} {x} {y} {dir:?}");
        }
    }

    Ok(())
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum PathState {
    New,
    Gone,
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Square {
    Unvisited,
    Visited,
    Wall,
    Guard,
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let disp = match self {
            Self::Unvisited => ' ',
            Self::Visited => 'X',
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
        let seen = HashSet::new();

        let mut processed_map = vec![vec![Square::Unvisited; map[0].len()]; map.len()];
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

        let (old_x, old_y) = self.guard_location;

        let mut new_x;
        let mut new_y;
        loop {
            let (dx, dy) = self.guard_direction.to_delta();

            new_x = (self.guard_location.0 as isize + dx) as usize;
            new_y = (self.guard_location.1 as isize + dy) as usize;

            if new_x >= self.height || new_y >= self.width {
                self.guard_direction = self.guard_direction.rotate_cw();
                return PathState::Gone;
            }

            if self.map[new_x][new_y] == Square::Wall {
                self.guard_direction = self.guard_direction.rotate_cw();
                continue;
            }

            break;
        }

        self.map[old_x][old_y] = Square::Visited;
        self.map[new_x][new_y] = Square::Guard;

        self.guard_location = (new_x, new_y);

        PathState::New
    }

    fn can_loop(&self, loop_dir: Direction, new_x: usize, new_y: usize) -> bool {
        let mut loop_dir = loop_dir.rotate_cw();

        let mut new_x = new_x;
        let mut new_y = new_y;
        let mut next_x;
        let mut next_y;

        loop {
            let (dx, dy) = loop_dir.to_delta();

            next_x = (new_x as isize + dx) as usize;
            next_y = (new_y as isize + dy) as usize;

            if next_x >= self.height || next_y >= self.width {
                break;
            }

            if self.map[next_x][next_y] == Square::Wall {
                break;
            }

            let new_tile = (next_x, next_y, loop_dir);

            if self.seen.contains(&new_tile) {
                return true;
            }

            new_x = next_x;
            new_y = next_y;
        }

        false
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
