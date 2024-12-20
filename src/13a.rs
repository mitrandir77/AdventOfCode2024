// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use std::{collections::BTreeSet, io::BufRead};
#[macro_use]
extern crate scan_rules;
use scan_rules::scan;

// algorithm from wikipedia: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
// Returns (GCD, BEZOUT_COEFFICIENTS
fn extended_euclid(a: i64, b: i64) -> (i64, (i64, i64)) {
    let (mut s, mut old_s) = (0, 1);
    let (mut t, mut old_t) = (1, 0);
    let (mut r, mut old_r) = (a, b);

    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }

    (old_r, (old_t, old_s))
}

// x and y such as a*x + by = c
fn solutions(a: i64, b: i64, c: i64) -> BTreeSet<(i64, i64)> {
    let mut res = BTreeSet::new();
    // gcd and coefficiencts
    let (gcd, (coa, cob)) = extended_euclid(a, b);
    if c % gcd != 0 {
        return res;
    }
    // initial solution in integers
    let m = c / gcd;
    let (mut x, mut y) = (coa * m, cob * m);

    if x < 0 {
        let m = -x / (b / gcd);
        x += m * (b / gcd);
        y -= m * (a / gcd);
    }

    while x <= 100 {
        if (0..=100).contains(&y) {
            res.insert((x, y));
        }
        x += b / gcd;
        y -= a / gcd;
    }
    if y < 0 {
        let m = -y / (a / gcd);
        x -= m * (b / gcd);
        y += m * (a / gcd);
    }
    while y <= 100 {
        if (0..=100).contains(&x) {
            res.insert((x, y));
        }
        x -= b / gcd;
        y += a / gcd;
    }
    res
}

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

        println!("A({ax}, {ay}) B({bx},{by}) P({px},{py})");

        let sol_x = solutions(ax, bx, px);
        let sol_y = solutions(ay, by, py);

        let sol: Vec<_> = sol_x.intersection(&sol_y).cloned().collect();
        dbg!(&sol);
        if sol.is_empty() {
            continue;
        }
        let mut min_cost = i64::MAX;
        for (a, b) in sol {
            if a * 3 + b < min_cost {
                min_cost = a * 3 + b;
            }
        }
        sum += min_cost;
    }

    println!("{sum}");
    Ok(())
}
