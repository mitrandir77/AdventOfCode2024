// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use itertools::Itertools;
use std::io::BufRead;
#[macro_use]
extern crate scan_rules;

fn run(mut a: u64, mut b: u64, mut c: u64, prog: &[u8]) -> Vec<u8> {
    let mut ip = 0;
    let mut out = vec![];
    loop {
        if ip >= prog.len() {
            break;
        }
        let opcode = prog[ip];
        let operand = prog[ip + 1] as u64;

        let combo = || match operand {
            0..=3 => operand,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!("bad combo operand {operand}"),
        };
        match opcode {
            0 => a >>= combo(),   // adv
            1 => b ^= operand,    // bxl
            2 => b = combo() % 8, // bst
            3 => {
                // jnz
                if a != 0 {
                    ip = operand as usize;
                    continue;
                }
            }
            4 => b ^= c,                        // bxc
            5 => out.push((combo() % 8) as u8), // out
            6 => b = a >> combo(),              // bdv
            7 => c = a >> combo(),              // cdv
            _ => panic!("bad opcode {opcode}"),
        }
        ip += 2;
    }
    out
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut lines = stdin.lock().lines();
    let line = lines.next().unwrap()?;
    let_scan!(line; ("Register A: ", let a: u64));

    let line = lines.next().unwrap()?;
    let_scan!(line; ("Register B: ", let b: u64));

    let line = lines.next().unwrap()?;
    let_scan!(line; ("Register C: ", let c: u64));

    let line = lines.nth(1).unwrap()?;
    let_scan!(line; ("Program: ", [ let prog: u8 ],+));
    let out = run(a, b, c, &prog);
    let out = out.iter().map(|n| n.to_string()).join(",");
    println!("{out}");
    Ok(())
}
