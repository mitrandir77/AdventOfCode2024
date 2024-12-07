// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
#[macro_use]
extern crate scan_rules;

use scan_rules::scan;
use std::io::BufRead;

fn concat_numbers(a: u64, b: u64) -> u64 {
    let mut pow = 10;
    while pow <= b {
        pow *= 10;
    }
    a * pow + b
}

fn fixable_recuse(tail: &[u64], sum: u64, target_sum: u64) -> bool {
    if sum > target_sum {
        return false;
    }
    if tail.is_empty() {
        return sum == target_sum;
    }
    // concat
    if fixable_recuse(&tail[1..], concat_numbers(sum, tail[0]), target_sum) {
        return true;
    }
    // multiply
    if fixable_recuse(&tail[1..], sum * tail[0], target_sum) {
        return true;
    }
    // add
    fixable_recuse(&tail[1..], sum + tail[0], target_sum)
}

fn fixable(components: &[u64], target_sum: u64) -> bool {
    fixable_recuse(&components[1..], components[0], target_sum)
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut sum = 0;
    for line in stdin.lock().lines() {
        let line = line?;
        scan!(&line;
            (let row_sum: u64, ":",  [ let components: u64 ]+) => {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        assert_eq!(concat_numbers(1, 2), 12);
        assert_eq!(concat_numbers(10, 2), 102);
        assert_eq!(concat_numbers(2, 10), 210);
    }
}
