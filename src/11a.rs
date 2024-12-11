// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use std::io::BufRead;

fn blink(stones: &[u64]) -> Vec<u64> {
    let mut res = vec![];

    for stone in stones {
        if *stone == 0 {
            res.push(1);
            continue;
        }

        let digits = stone.to_string();
        let len = digits.len();
        if len % 2 == 0 {
            let left = digits[0..len / 2].parse().unwrap();
            let right = digits[len / 2..len].parse().unwrap();
            res.push(left);
            res.push(right);
            continue;
        }

        res.push(*stone * 2024);
    }
    res
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

    for _i in 0..25 {
        stones = blink(&stones);
    }

    println!("{}", stones.len());
    Ok(())
}
