use std::io;

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let mut line = String::new();

    io::stdin().read_line(&mut line)?;

    let line = line.trim();

    let mut stones: Vec<u64> = line
        .split_whitespace()
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();

    for i in 0..25 {
        stones = blink(stones);
        println!("{} - {}", i, stones.len());
    }

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

fn blink(stones: Vec<u64>) -> Vec<u64> {
    let mut new_stones = vec![];
    for stone in stones {
        match process_stone(stone) {
            Whoops::Single(s) => new_stones.push(s),
            Whoops::Double(l, r) => {
                new_stones.push(l);
                new_stones.push(r);
            }
        }
    }
    new_stones
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
