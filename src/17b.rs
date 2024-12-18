// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
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

// recursive body of find_a
fn _find_a_internal(mut a: u64, digit: usize, out: &[u8]) -> Vec<u64> {
    let mut res = vec![];
    a *= 8;
    for i in 0..8 {
        let cand = a + i;
        let cand_out = run(cand, 0, 0, out);
        if cand_out[0] == out[digit] {
            if digit > 0 {
                res.extend(_find_a_internal(cand, digit - 1, out));
            }
            if digit == 0 {
                res.push(cand);
            }
        }
    }
    res
}

// Finds a so the out equal param, assuming the program is:
// ```
// while a != 0 {
//     b = a % 8;
//     b^=2;
//     c = a >> b;
//     b^=3;
//     b^=c;
//     out.push(b % 8);
//     a >>= 3;
// }
// ```
fn find_a(out: &[u8]) -> Option<u64> {
    _find_a_internal(0, out.len() - 1, out).into_iter().min()
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut lines = stdin.lock().lines();
    let line = lines.next().unwrap()?;
    let_scan!(line; ("Register A: ", let _a: u64));

    let line = lines.next().unwrap()?;
    let_scan!(line; ("Register B: ", let b: u64));

    let line = lines.next().unwrap()?;
    let_scan!(line; ("Register C: ", let c: u64));

    let line = lines.nth(1).unwrap()?;
    let_scan!(line; ("Program: ", [ let prog: u8 ],+));

    let a = find_a(&prog).unwrap();
    // dbg!(run(a, b, c, &prog) == prog,);
    println!("{a}");

    Ok(())
}
