// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use std::io::BufRead;
#[macro_use]
extern crate scan_rules;
use scan_rules::scan;

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut lines_iter = stdin.lock().lines();
    let mut sum = 0;
    while let Some(line) = lines_iter.next() {
        let line = line?;
        let_scan!(line; ("Button A: X+", let ax: i64, ", Y+", let ay: i64));
        let line = lines_iter.next().unwrap()?;
        let_scan!(line; ("Button B: X+", let bx: i64, ", Y+", let by: i64));
        let line = lines_iter.next().unwrap()?;
        let_scan!(line; ("Prize: X=", let px: i64, ", Y=", let py: i64));
        lines_iter.next();

        let px = px + 10000000000000;
        let py = py + 10000000000000;

        // println!("A({ax}, {ay}) B({bx},{by}) P({px},{py})");

        // ax * a + bx * b = px;
        // ay * a + by * b = py;
        let b = (ax * py - ay * px) / (ax * by - ay * bx);
        let a = (px - bx * b) / ax;

        if ax * a + bx * b == px && ay * a + by * b == py {
            sum += 3 * a + b;
        }
    }

    println!("{sum}");
    Ok(())
}
