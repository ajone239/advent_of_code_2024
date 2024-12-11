use std::{collections::VecDeque, io};

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut line = String::new();

    stdin.read_line(&mut line)?;

    let line: Vec<_> = line.trim().chars().collect();

    let mut file_blocks = vec![];
    let mut empty_blocks: VecDeque<EmptyBlock> = VecDeque::new();

    let mut index = 0;
    for (id, chunk) in line.chunks(2).enumerate() {
        let file_size = chunk[0] as usize - '0' as usize;

        let file_block = FileBlock {
            id,
            length: file_size,
            index,
        };

        file_blocks.push(file_block);

        index += file_size;

        if chunk.len() <= 1 {
            continue;
        }

        let space_size = chunk[1] as usize - '0' as usize;

        if space_size <= 0 {
            continue;
        }

        let empty_block = EmptyBlock {
            length: space_size,
            index,
        };
        empty_blocks.push_back(empty_block);

        index += space_size
    }

    let mut new_file_blocks = vec![];

    while empty_blocks.len() > 0 {
        let Some(mut empty_block) = empty_blocks.pop_front() else {
            break;
        };

        while empty_block.length > 0 {
            for i in (0..file_blocks.len()).rev() {
                let last_file_block = &mut file_blocks[i];
                if last_file_block.index < empty_block.index {
                    empty_block.length = 0;
                    break;
                }

                match (empty_block.length, last_file_block.length) {
                    (elen, flen) if elen >= flen => {
                        let new_file_block = FileBlock {
                            id: last_file_block.id,
                            length: flen,
                            index: empty_block.index,
                        };

                        new_file_blocks.push(new_file_block);

                        empty_block.length -= flen;
                        empty_block.index += flen;

                        file_blocks.remove(i);

                        break;
                    }
                    (_, _) => (),
                }
            }
        }
    }
    file_blocks.append(&mut new_file_blocks);
    file_blocks.sort_by(|s, o| s.index.cmp(&o.index));

    let result: usize = file_blocks
        .iter()
        .flat_map(|fb| {
            let from = fb.index;
            let to = fb.index + fb.length;

            (from..to).map(|i| fb.id * i)
        })
        .sum();

    println!("{}", result);

    Ok(())
}

#[derive(Debug)]
struct FileBlock {
    id: usize,
    length: usize,
    index: usize,
}

#[derive(Debug)]
struct EmptyBlock {
    length: usize,
    index: usize,
}
