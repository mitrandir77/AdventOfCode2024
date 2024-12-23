// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use std::io::BufRead;

fn next_secret_number(mut secret: u64) -> u64 {
    secret = secret ^ (secret * 64);
    secret %= 16777216;
    secret = secret ^ (secret / 32);
    secret %= 16777216;
    secret = secret ^ (secret * 2048);
    secret %= 16777216;
    secret
}

fn nth_secret_number(mut seed: u64, n: u64) -> u64 {
    for _i in 0..n {
        seed = next_secret_number(seed);
    }
    seed
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut res = 0;
    for line in stdin.lock().lines() {
        let line = line?;
        let seed = line.parse()?;
        // dbg!(&seed);
        let secret = nth_secret_number(seed, 2000);
        // dbg!(&secret);
        res += secret;
    }
    println!("{res}");

    Ok(())
}
