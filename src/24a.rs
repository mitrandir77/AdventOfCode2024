// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use itertools::Itertools;
use scan_rules::scanner::Word;
use std::{collections::HashMap, io::BufRead};
#[macro_use]
extern crate scan_rules;

#[derive(Debug)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn apply(&self, a: u8, b: u8) -> u8 {
        match self {
            Op::And => a & b,
            Op::Or => a | b,
            Op::Xor => a ^ b,
        }
    }
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut lines = stdin.lock().lines();
    let mut values = HashMap::new();
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }
        let_scan!(line; (let wire: Word<String>, ":", let val: u8));
        values.insert(wire, val);
    }
    let mut rules = HashMap::new();
    while let Some(Ok(line)) = lines.next() {
        scan!(&line;
            (let a: Word<String>, "AND",  let b: Word<String>, "->", let c: Word<String>) => {
                rules.insert(c, (Op::And, a, b));
            },
            (let a: Word<String>, "OR",  let b: Word<String>, "->", let c: Word<String>) => {
                rules.insert(c, (Op::Or, a, b));
            },
            (let a: Word<String>, "XOR",  let b: Word<String>, "->", let c: Word<String>) => {
                rules.insert(c, (Op::Xor, a, b));
            },
        )
        .unwrap();
    }

    let mut zs = rules
        .keys()
        .filter(|k| k.starts_with("z"))
        .cloned()
        .collect_vec();
    zs.sort();
    let mut stack = zs.clone();
    while let Some(wire) = stack.pop() {
        // dbg!(&wire);
        if values.contains_key(&wire) {
            // dbg!("cont");
            continue;
        }
        if let Some((op, a, b)) = rules.get(&wire) {
            // dbg!((op, a, b));
            match (values.get(a), values.get(b)) {
                (Some(a), Some(b)) => {
                    let val = op.apply(*a, *b);
                    values.insert(wire.clone(), val);
                }
                (Some(_a), None) => {
                    stack.push(wire.to_string());
                    stack.push(b.to_string());
                }
                (None, Some(_b)) => {
                    stack.push(wire.to_string());
                    stack.push(a.to_string());
                }
                (None, None) => {
                    stack.push(wire.to_string());
                    stack.push(a.to_string());
                    stack.push(b.to_string());
                }
            }
        } else {
            panic!("no rule for {wire}");
        }
    }

    let mut res: u64 = 0;
    for z in zs.into_iter().rev() {
        // dbg!(&z, values.get(&z).unwrap());
        res <<= 1;
        res += *values.get(&z).unwrap() as u64;
    }
    println!("{res}");
    Ok(())
}
