// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use std::io::BufRead;

#[derive(Debug)]
enum Device {
    Key(Vec<u8>),
    Lock(Vec<u8>),
}

fn parse(schema: Vec<String>) -> Device {
    let mut res = vec![0; 5];
    for row in schema.iter() {
        for (i, c) in row.chars().enumerate() {
            if c == '#' {
                res[i] += 1;
            }
        }
    }
    for c in res.iter_mut() {
        *c -= 1;
    }
    if schema[0] == "#####" {
        Device::Lock(res)
    } else {
        Device::Key(res)
    }
}

fn overlap(k: &[u8], l: &[u8]) -> bool {
    for (k, l) in k.iter().zip(l.iter()) {
        if k + l >= 6 {
            return true;
        }
    }
    false
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut current_key_or_lock = vec![];
    let mut devices = vec![];
    for line in stdin.lock().lines() {
        let line = line?;
        if line.is_empty() {
            devices.push(parse(current_key_or_lock));
            current_key_or_lock = vec![];
        } else {
            current_key_or_lock.push(line);
        }
    }
    devices.push(parse(current_key_or_lock));

    let mut keys = vec![];
    let mut locks = vec![];
    for device in devices {
        match device {
            Device::Key(k) => keys.push(k),
            Device::Lock(l) => locks.push(l),
        }
    }
    let mut sum = 0;
    for k in &keys {
        for l in &locks {
            if !overlap(k, l) {
                sum += 1;
            }
        }
    }
    println!("{}", sum);

    Ok(())
}
