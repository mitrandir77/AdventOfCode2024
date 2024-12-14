// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use std::{io::BufRead, thread, time::Duration};
#[macro_use]
extern crate scan_rules;

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut lines = stdin.lock().lines();
    let_scan!(lines.next().unwrap()?; (let w: i64, ",", let h: i64));
    let mut robots = vec![];
    for line in lines {
        let line = line?;
        let_scan!(line; ("p=", let x: i64, ",", let y: i64, "v=", let vx: i64, ",", let vy: i64));

        robots.push((x, y, vx, vy));
    }
    // interesting at 20
    // interesting at 121
    // (I'll set to step through those in case those cycle)
    let start_iter = 121;
    let step = 101;
    for (x, y, vx, vy) in robots.iter_mut() {
        (*x, *y) = (
            (*x + start_iter * (*vx + w)) % w,
            (*y + start_iter * (*vy + h)) % h,
        );
    }
    for i in (start_iter..).step_by(step as usize) {
        let mut fb = vec![vec![' '; w as usize]; h as usize];

        println!("Turn {i}\n\n");
        for (x, y, vx, vy) in robots.iter_mut() {
            fb[*y as usize][*x as usize] = '#';
            (*x, *y) = ((*x + step * (*vx + w)) % w, (*y + step * (*vy + h)) % h);
        }

        for line in fb.iter() {
            for c in line.iter() {
                print!("{}", c);
            }
            println!();
        }

        // thread::sleep(Duration::from_millis(300));
        thread::sleep(Duration::from_millis(1000));

        print!("{}[2J", 27 as char);
    }

    Ok(())
}
