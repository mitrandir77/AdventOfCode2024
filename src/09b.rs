// Advent of Code 2024
// (c) 2024 Mateusz Kwapich
#![feature(btree_cursors)]

use anyhow::Result;
use std::collections::BTreeSet;
use std::io::BufRead;
use std::ops::Bound;

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

#[allow(dead_code)]
fn print_blocks(blocks: &[Block]) {
    for block in blocks {
        for _i in 0..block.len {
            print!(
                "{}",
                block
                    .file_id
                    .map(|id| id.to_string())
                    .unwrap_or(".".to_string())
            )
        }
    }
    println!();
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let line = stdin.lock().lines().next().unwrap()?;
    let mut blocks = vec![];
    let mut by_length = BTreeSet::new();
    for (pos, len) in line.chars().enumerate() {
        let file_id = if pos % 2 == 0 { Some(pos / 2) } else { None };
        let len = len.to_digit(10).unwrap();
        blocks.push(Block { len, file_id });
        if file_id.is_some() {
            by_length.insert((len, blocks.len() - 1));
        }
    }

    let mut compacted = Vec::new();

    let mut i = 0;
    while i < blocks.len() {
        if blocks[i].file_id.is_some() {
            by_length.remove(&(blocks[i].len, i));
            compacted.push(blocks[i].clone());
            i += 1;
            continue;
        }
        if blocks[i].len == 0 {
            i += 1;
            continue;
        }

        // Blocks that will fit into free space at the iterator
        let mut fitting_blocks = by_length.upper_bound(Bound::Excluded(&(blocks[i].len + 1, 0)));

        // Find rightmost fitting block
        let mut rightmost = if let Some(fitting) = fitting_blocks.prev() {
            *fitting
        } else {
            // If none the space will stay empty
            compacted.push(blocks[i].clone());
            i += 1;
            continue;
        };
        while let Some((fitting_len, fitting_pos)) = fitting_blocks.prev() {
            if *fitting_pos > rightmost.1 {
                rightmost = (*fitting_len, *fitting_pos);
            }
        }

        // Move the found block to the location
        by_length.remove(&rightmost);
        compacted.push(blocks[rightmost.1].clone());
        blocks[i].len -= rightmost.0;
        blocks[rightmost.1].file_id = None;
    }

    let sum = checksum(&compacted);
    // print_blocks(&compacted);
    println!("{sum}");

    Ok(())
}
