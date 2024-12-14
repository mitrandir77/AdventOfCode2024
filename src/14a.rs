// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use std::{cmp::Ordering, io::BufRead};
#[macro_use]
extern crate scan_rules;

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut lines = stdin.lock().lines();
    let_scan!(lines.next().unwrap()?; (let w: i64, ",", let h: i64));
    let mut sum = vec![vec![0_i64; 2]; 2];
    let turns = 100;
    for line in lines {
        let line = line?;
        let_scan!(line; ("p=", let x: i64, ",", let y: i64, "v=", let vx: i64, ",", let vy: i64));

        let (nx, ny) = (
            (x + vx * turns + turns * w) % w,
            (y + vy * turns + turns * h) % h,
        );
        // dbg!(x, y, vx, vy);
        match (nx.cmp(&(w / 2)), ny.cmp(&(h / 2))) {
            (Ordering::Less, Ordering::Less) => {
                sum[0][0] += 1;
            }
            (Ordering::Less, Ordering::Greater) => {
                sum[0][1] += 1;
            }
            (Ordering::Greater, Ordering::Less) => {
                sum[1][0] += 1;
            }
            (Ordering::Greater, Ordering::Greater) => {
                sum[1][1] += 1;
            }
            _ => {}
        }
    }

    // dbg!(&sum);
    let sum: i64 = sum.iter().flatten().product();
    println!("{sum}");
    Ok(())
}
