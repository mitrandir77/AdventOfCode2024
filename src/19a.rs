// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use scan_rules::scanner::Word;
use std::io::BufRead;
#[macro_use]
extern crate scan_rules;

fn possible(towels: &[String], design: &str) -> bool {
    let mut cache = vec![false; design.len() + 1];
    cache[design.len()] = true;

    'outer: for start in (0..design.len()).rev() {
        for towel in towels {
            // dbg!(&design[start..]);
            if design[start..].starts_with(towel) && cache[start + towel.len()] {
                cache[start] = true;
                continue 'outer;
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
        if possible(&towels, &design) {
            sum += 1;
        }
    }
    println!("{sum}");

    Ok(())
}
