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
        let mut seq: Vec<i64> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .rev()
            .collect();

        if check(&seq, 1) {
            sum += 1;
        } else if seq.len() > 1 {
            let (_first, rest) = seq.split_first().unwrap();
            // edge case 1: we don't allow option to skip the first element in
            // the check function let's run another check without it but with
            // lower dampen limit
            if check(rest, 0) {
                sum += 1;
            } else {
                // edge case 2: even if first two element are making safe
                // sequence we may want to skip the second one so avoid forcing
                // the ascending/descending order for the rest of sequence
                seq[1] = seq[0];
                let (_first, rest) = seq.split_first().unwrap();
                if check(rest, 0) {
                    sum += 1;
                }
            }
        }
    }

    println!("{sum}");
    Ok(())
}
