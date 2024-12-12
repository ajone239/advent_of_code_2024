use std::{collections::HashMap, io};

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let mut line = String::new();

    io::stdin().read_line(&mut line)?;

    let line = line.trim();

    let stones: Vec<u64> = line
        .split_whitespace()
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut seen = HashMap::new();
    let result: usize = stones.into_iter().map(|s| blink(s, &mut seen, 75)).sum();

    println!("{}", result);

    Ok(())
}

/*
If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.

If the stone is engraved with a number that has an even number of digits, it is replaced by two stones.
    The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)

If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
*/

enum Whoops<T> {
    Single(T),
    Double(T, T),
}

fn blink(stone: u64, seen: &mut HashMap<(u64, usize), usize>, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    if let Some(count) = seen.get(&(stone, depth)) {
        return *count;
    }

    let stone_count = match process_stone(stone) {
        Whoops::Single(s) => blink(s, seen, depth - 1),
        Whoops::Double(l, r) => blink(l, seen, depth - 1) + blink(r, seen, depth - 1),
    };

    seen.insert((stone, depth), stone_count);

    stone_count
}

fn process_stone(stone: u64) -> Whoops<u64> {
    if stone == 0 {
        return Whoops::Single(1);
    }

    let s_str = stone.to_string();

    if s_str.len() & 1 == 1 {
        return Whoops::Single(stone * 2024);
    }

    let chars: Vec<char> = s_str.chars().collect();

    let left: String = chars[..chars.len() / 2].iter().collect();
    let right: String = chars[chars.len() / 2..].iter().collect();

    let left = left.parse().unwrap();
    let right = right.parse().unwrap();

    Whoops::Double(left, right)
}
