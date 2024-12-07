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
        let line = line?.trim().to_string();

        if line.is_empty() {
            break;
        }

        lines.push(line.chars().collect());
    }

    let mut map = Map::new(lines);

    let init_local = map.guard_location.clone();

    // let mut i = 0;
    while PathState::New == map.advance() {
        let mut blah = String::new();
        let _ = io::stdin().read_line(&mut blah);
        // i += 1;
        // println!("{}", i);
    }

    println!("{:?}", map);

    println!("{}", map.obstacles.contains(&init_local));
    println!("can loop count: {}", map.obstacles.len());

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
            Self::Unvisited => '.',
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
    init_direction: Direction,
    guard_location: (usize, usize),
    init_location: (usize, usize),
    seen: HashSet<(usize, usize, Direction)>,
    obstacles: HashSet<(usize, usize)>,
}

impl Map {
    fn new(map: Vec<Vec<char>>) -> Self {
        let width = map[0].len();
        let height = map.len();
        let seen = HashSet::new();
        let obstacles = HashSet::new();

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
            init_location: guard_location,
            init_direction: guard_direction,
            seen,
            obstacles,
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
                return PathState::Gone;
            }

            if self.map[new_x][new_y] == Square::Wall {
                self.guard_direction = self.guard_direction.rotate_cw();
                continue;
            }

            break;
        }

        self.map[new_x][new_y] = Square::Wall;

        if self.can_loop(self.guard_direction, old_x, old_y) {
            self.obstacles.insert((new_x, new_y));
        }

        self.map[old_x][old_y] = Square::Visited;
        self.map[new_x][new_y] = Square::Guard;

        self.guard_location = (new_x, new_y);

        PathState::New
    }

    fn can_loop(&self, init_dir: Direction, new_x: usize, new_y: usize) -> bool {
        let mut loop_dir = self.init_direction;
        let (mut new_x, mut new_y) = self.init_location;
        let mut next_x;
        let mut next_y;

        let mut seen_internal = HashSet::new();

        loop {
            let (dx, dy) = loop_dir.to_delta();

            next_x = (new_x as isize + dx) as usize;
            next_y = (new_y as isize + dy) as usize;

            if next_x >= self.height || next_y >= self.width {
                return false;
            }

            if self.map[next_x][next_y] == Square::Wall {
                loop_dir = loop_dir.rotate_cw();

                continue;
            }

            let new_tile = (new_x, new_y, loop_dir);

            if !seen_internal.insert(new_tile) {
                return true;
            };

            new_x = next_x;
            new_y = next_y;
        }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.guard_direction)?;
        writeln!(f, "{:?}", self.guard_location)?;
        writeln!(f, "{:?}", self.obstacles.len())?;
        // writeln!(f, "{:?}", self.seen)?;

        for (i, row) in self.map.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if self.obstacles.contains(&(i, j)) {
                    write!(f, "0")?;
                } else {
                    write!(f, "{:?}", cell)?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
