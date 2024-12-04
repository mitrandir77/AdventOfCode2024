// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use itertools::Itertools;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut lines = vec![];
    for line in stdin.lock().lines() {
        let line = line?;
        lines.push(line.chars().collect_vec());
    }

    let h = lines.len();
    let w = lines[0].len();

    let mut sum = 0;
    for i in 1..(h - 1) {
        for j in 1..(w - 1) {
            let mut cnt = 0;
            if lines[i][j] == 'A' {
                if lines[i - 1][j - 1] == 'M' || lines[i + 1][j + 1] == 'M' {
                    cnt += 1;
                }
                if lines[i - 1][j - 1] == 'S' || lines[i + 1][j + 1] == 'S' {
                    cnt += 1;
                }
                if lines[i - 1][j + 1] == 'M' || lines[i + 1][j - 1] == 'M' {
                    cnt += 1;
                }
                if lines[i - 1][j + 1] == 'S' || lines[i + 1][j - 1] == 'S' {
                    cnt += 1;
                }
            }
            if cnt == 4 {
                sum += 1;
            }
        }
    }

    println!("{sum}");
    Ok(())
}
