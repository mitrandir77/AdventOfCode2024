// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use itertools::Itertools;
use std::{collections::BTreeMap, io::BufRead};

fn next_secret_number(mut secret: i64) -> i64 {
    secret = secret ^ (secret * 64);
    secret %= 16777216;
    secret = secret ^ (secret / 32);
    secret %= 16777216;
    secret = secret ^ (secret * 2048);
    secret %= 16777216;
    secret
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut all_monkeys = BTreeMap::new();
    for line in stdin.lock().lines() {
        let line = line?;
        let mut seed: i64 = line.parse()?;
        let mut current_monkey = BTreeMap::new();
        let secret_numbers = std::iter::from_fn(move || {
            seed = next_secret_number(seed);
            Some(seed)
        });
        for (a, b, c, d, e) in secret_numbers.take(2000).tuple_windows() {
            let cur = e;
            let (a, b, c, d, e) = (
                (a % 10) as i8,
                (b % 10) as i8,
                (c % 10) as i8,
                (d % 10) as i8,
                (e % 10) as i8,
            );
            let diffs: (i8, i8, i8, i8) = (b - a, c - b, d - c, e - d);
            current_monkey.entry(diffs).or_insert(cur % 10);
        }

        for (diffs, val) in current_monkey {
            let all_val: &mut i64 = all_monkeys.entry(diffs).or_default();
            *all_val += val;
        }
    }
    let max = all_monkeys.values().max().unwrap();
    println!("{max}");

    Ok(())
}
