// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use std::cmp::Ordering;
use std::io::BufRead;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Order {
    Decreasing,
    Increasing,
}

fn check(seq: &[i64], dampen_limit: u64) -> bool {
    let (mut prev, rest) = seq.split_first().unwrap();

    let mut dampen_count = 0;
    let mut order = None;

    for n in rest {
        if (n - prev).abs() > 3 {
            dampen_count += 1;
            continue;
        }

        match n.cmp(prev) {
            Ordering::Greater => {
                if order == Some(Order::Increasing) || order.is_none() {
                    order = Some(Order::Increasing);
                } else {
                    dampen_count += 1;
                    continue;
                }
            }
            Ordering::Less => {
                if order == Some(Order::Decreasing) || order.is_none() {
                    order = Some(Order::Decreasing);
                } else {
                    dampen_count += 1;
                    continue;
                }
            }
            Ordering::Equal => {
                dampen_count += 1;
                continue;
            }
        }

        prev = n;
    }
    dampen_count <= dampen_limit
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut sum = 0;
    for line in stdin.lock().lines() {
        let line = line?;
        let seq: Vec<i64> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .rev()
            .collect();

        if check(&seq, 0) {
            sum += 1;
        }
    }

    println!("{sum}");
    Ok(())
}
