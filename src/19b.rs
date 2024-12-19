// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use scan_rules::scanner::Word;
use std::io::BufRead;
#[macro_use]
extern crate scan_rules;

fn ways_to_arrange(towels: &[String], design: &str) -> u64 {
    let mut cache = vec![0; design.len() + 1];
    cache[design.len()] = 1;

    for start in (0..design.len()).rev() {
        for towel in towels {
            // dbg!(&design[start..]);
            if design[start..].starts_with(towel) {
                cache[start] += cache[start + towel.len()];
            }
        }
    }
    // dbg!(&cache);
    cache[0]
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut lines = stdin.lock().lines();
    let line = lines.next().unwrap()?;
    let_scan!(line; ([ let towels: Word<String>],+));

    let mut sum = 0;
    for line in lines.skip(1) {
        let design = line?;
        sum += ways_to_arrange(&towels, &design);
    }
    println!("{sum}");

    Ok(())
}
