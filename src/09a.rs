// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use std::cmp;
use std::io::BufRead;

#[derive(Debug, Clone)]
struct Block {
    len: u32,
    file_id: Option<usize>,
}

fn checksum(blocks: &[Block]) -> usize {
    let mut sum = 0;
    let mut len = 0;
    for b in blocks.iter() {
        sum += b.file_id.unwrap_or(0) * (2 * len + b.len as usize - 1) * b.len as usize / 2;
        len += b.len as usize;
    }
    sum
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let line = stdin.lock().lines().next().unwrap()?;
    let mut blocks = vec![];
    for (pos, len) in line.chars().enumerate() {
        let file_id = if pos % 2 == 0 { Some(pos / 2) } else { None };
        let len = len.to_digit(10).unwrap();
        blocks.push(Block { len, file_id });
    }

    let mut front = 0;
    let mut back = blocks.len() - 1;

    let mut compacted = Vec::new();

    loop {
        if blocks[front].file_id.is_some() {
            compacted.push(blocks[front].clone());
            front += 1;
        }
        if blocks[front].len == 0 {
            front += 1;
            continue;
        }
        if blocks[back].file_id.is_none() || blocks[back].len == 0 {
            back -= 1;
            continue;
        }
        if front >= back {
            break;
        }

        let new_len = cmp::min(blocks[back].len, blocks[front].len);

        blocks[back].len -= new_len;
        blocks[front].len -= new_len;

        compacted.push(Block {
            len: new_len,
            file_id: blocks[back].file_id,
        });
    }

    let sum = checksum(&compacted);
    // dbg!(compacted);
    println!("{sum}");

    Ok(())
}
