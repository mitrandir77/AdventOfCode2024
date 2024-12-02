// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
#[macro_use]
extern crate scan_rules;
use scan_rules::scan;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut list_a = vec![];
    let mut list_b = vec![];
    for line in stdin.lock().lines() {
        let line = line?;
        scan!(&line;
            (let num_a: i64,  let num_b: i64) => {
                list_a.push(num_a);
                list_b.push(num_b);
            },
        )
        .unwrap();
    }
    list_a.sort();
    list_b.sort();
    let mut sum = 0;
    for (a, b) in list_a.iter().zip(list_b.iter()) {
        sum += (a - b).abs();
    }
    println!("{sum}");
    Ok(())
}
