// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
#[macro_use]
extern crate scan_rules;

use scan_rules::scan;
use std::io::BufRead;

fn fixable_recuse(tail: &[i64], sum: i64, target_sum: i64) -> bool {
    if sum > target_sum {
        return false;
    }
    if tail.is_empty() {
        return sum == target_sum;
    }
    let add = fixable_recuse(&tail[1..], sum + tail[0], target_sum);
    let multiply = fixable_recuse(&tail[1..], sum * tail[0], target_sum);
    add || multiply
}

fn fixable(components: &[i64], target_sum: i64) -> bool {
    fixable_recuse(&components[1..], components[0], target_sum)
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut sum = 0;
    for line in stdin.lock().lines() {
        let line = line?;
        scan!(&line;
            (let row_sum: i64, ":",  [ let components: i64 ]+) => {
                if fixable(&components, row_sum) {
                    sum+=row_sum;
                }
            },
        )
        .unwrap();
    }
    println!("{sum}");
    Ok(())
}
