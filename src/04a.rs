// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use itertools::Itertools;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut lines = vec![];
    let mut horizontal = String::new();
    for line in stdin.lock().lines() {
        let line = line?;
        lines.push(line.chars().collect_vec());
        horizontal.push_str(&line);
        horizontal.push('\n');
    }
    let h = lines.len();
    let w = lines[0].len();

    let mut vertical = String::new();
    for i in 0..h {
        for line in lines.iter() {
            vertical.push(line[i]);
        }
        vertical.push('\n');
    }

    let mut diag1 = String::new();
    for i in 0..(h + w - 1) {
        for j in (0..=i).rev() {
            let c = lines.get(j).map(|l| l.get(i - j));
            if let Some(Some(c)) = c {
                diag1.push(*c);
            }
        }
        diag1.push('\n');
    }

    let mut diag2 = String::new();
    for i in 0..(h + w - 1) {
        for j in (0..=i).rev() {
            let c = lines.get(j).map(|l| l.get(w - i + j));
            if let Some(Some(c)) = c {
                diag2.push(*c);
            }
        }
        diag2.push('\n');
    }

    let mut sum = 0;
    for s in [horizontal, vertical, diag1, diag2] {
        sum += s.matches("XMAS").count();
        sum += s.matches("SAMX").count();
    }
    println!("{sum}");
    Ok(())
}
