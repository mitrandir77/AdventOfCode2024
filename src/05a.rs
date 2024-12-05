// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use std::{collections::BTreeSet, io::BufRead};
#[macro_use]
extern crate scan_rules;
use scan_rules::scan;

fn validate(rules: &BTreeSet<(i64, i64)>, pages: &[i64]) -> bool {
    for w in pages.windows(2) {
        let (a, b) = (w[0], w[1]);
        if rules.contains(&(b, a)) {
            return false;
        }
    }
    true
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut rules = BTreeSet::new();
    for line in stdin.lock().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        scan!(&line;
            (let num_a: i64, "|", let num_b: i64) => {
                rules.insert((num_a, num_b));
            },
        )
        .unwrap();
    }

    let mut sum = 0;
    for line in stdin.lock().lines() {
        let line = line?;
        let pages = line
            .split(',')
            .map(|n| n.parse::<i64>())
            .collect::<Result<Vec<_>, std::num::ParseIntError>>()?;

        if validate(&rules, &pages) {
            sum += pages[pages.len() / 2];
        }
    }
    println!("{sum}");
    Ok(())
}
