// Advent of Code 2023
// (c) 2023 Mateusz Kwapich

use anyhow::Result;
#[macro_use]
extern crate scan_rules;
use mset::MultiSet;

use scan_rules::scan;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut list_a = vec![];
    let mut map_b = MultiSet::new();
    for line in stdin.lock().lines() {
        let line = line?;
        scan!(&line;
            (let num_a: i64,  let num_b: i64) => {
                list_a.push(num_a);
                map_b.insert(num_b);
            },
        )
        .unwrap();
    }
    let mut sum = 0;
    for a in list_a.iter() {
        let occurences = map_b.get(a).unwrap_or(0);
        sum += a * (occurences as i64);
    }
    println!("{sum}");
    Ok(())
}
