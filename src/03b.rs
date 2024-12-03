// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use regex::Regex;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let re = Regex::new(r"(?P<mul>mul\((\d+),(\d+)\))|(?P<dont>don't\(\))|(?P<do>do\(\))").unwrap();
    let mut sum: i64 = 0;

    let mut toggle = true;
    for line in stdin.lock().lines() {
        let line = line?;
        for caps in re.captures_iter(&line) {
            if let Some(_link_match) = caps.name("mul") {
                if toggle {
                    sum += caps[2].parse::<i64>()? * caps[3].parse::<i64>()?;
                }
            }

            if let Some(_link_match) = caps.name("do") {
                toggle = true;
            }
            if let Some(_link_match) = caps.name("dont") {
                toggle = false;
            }
        }
    }
    println!("{}", sum);
    Ok(())
}
