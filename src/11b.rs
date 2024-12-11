// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use memoize::memoize;
use std::io::BufRead;

#[memoize]
fn num_stones(stone: u64, iter: usize) -> usize {
    if iter == 0 {
        return 1;
    }

    if stone == 0 {
        return num_stones(1, iter - 1);
    }

    let digits = stone.to_string();
    let len = digits.len();
    if len % 2 == 0 {
        let left = digits[0..len / 2].parse().unwrap();
        let right = digits[len / 2..len].parse().unwrap();

        return num_stones(left, iter - 1) + num_stones(right, iter - 1);
    }

    num_stones(stone * 2024, iter - 1)
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut stones = vec![];
    for line in stdin.lock().lines() {
        let line = line?;
        for num in line.split(" ") {
            let num: u64 = num.parse()?;
            stones.push(num);
        }
    }

    let mut res = 0;
    for stone in stones {
        res += num_stones(stone, 75);
    }

    println!("{}", res);
    Ok(())
}
