// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use regex::Regex;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum: i64 = 0;

    for line in stdin.lock().lines() {
        let line = line?;
        for caps in re.captures_iter(&line) {
            sum += caps[1].parse::<i64>()? * caps[2].parse::<i64>()?;
        }
    }
    println!("{}", sum);
    Ok(())
}
