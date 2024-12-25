// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

// WARNING: code is ugly - didn't have time to make it pretty

#![feature(map_try_insert)]
use anyhow::Result;
use scan_rules::scanner::Word;
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};
#[macro_use]
extern crate scan_rules;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    And,
    Or,
    Xor,
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
                if a< b{
                    rules.insert(c, (Op::And, a, b));
                }
                else {
                    rules.insert(c, (Op::And, b, a));
                }
            },
            (let a: Word<String>, "OR",  let b: Word<String>, "->", let c: Word<String>) => {
                if a< b{
                    rules.insert(c, (Op::Or, a, b));
                }
                else {
                    rules.insert(c, (Op::Or, b, a));
                }
            },
            (let a: Word<String>, "XOR",  let b: Word<String>, "->", let c: Word<String>) => {
                if a< b{
                    rules.insert(c, (Op::Xor, a, b));
                }
                else {
                    rules.insert(c, (Op::Xor, b, a));
                }
            },
        )
        .unwrap();
    }
    let mut aliases = HashMap::new();
    let mut rev_aliases = HashMap::new();

    let len = values.len() / 2;
    let mut bad = HashSet::new();
    for (res, (op, a, b)) in rules.iter() {
        if a.starts_with("x") && b.starts_with("y") {
            let i: usize = a[1..].parse()?;
            let j: usize = b[1..].parse()?;
            if i != j {
                panic!("input not matching assuptions!");
            }
            match *op {
                Op::Xor => {
                    aliases.insert(res.clone(), format!("s{:02}", j));
                    rev_aliases.insert(format!("s{:02}", j), res.clone());
                }
                Op::And => {
                    if j == 0 {
                        aliases.insert(res.clone(), "c00".to_string());
                        rev_aliases.insert("c00".to_string(), res.clone());
                    } else {
                        aliases.insert(res.clone(), format!("o{:02}", j));
                        rev_aliases.insert(format!("o{:02}", j), res.clone());
                    }
                }
                Op::Or => panic!("unexpected op"),
            }
        }
    }
    for i in 1..len {
        let c_alias = format!("c{:02}", i - 1);
        let s_alias = format!("s{:02}", i);
        let a_alias = format!("a{:02}", i);
        let o_alias = format!("o{:02}", i);
        let z_alias = format!("z{:02}", i);
        for (res, (op, a, b)) in rules.iter() {
            match (aliases.get(a), aliases.get(b)) {
                (Some(aa), Some(ba))
                    if (*aa == c_alias && *ba == s_alias) || (*aa == s_alias && *ba == c_alias) =>
                {
                    match *op {
                        Op::Xor => {
                            aliases.insert(res.clone(), z_alias.clone());
                            rev_aliases.insert(z_alias.clone(), res.clone());
                        }
                        Op::And => {
                            aliases.insert(res.clone(), a_alias.clone());
                            rev_aliases.insert(a_alias.clone(), res.clone());
                        }
                        Op::Or => {}
                    }
                }
                (Some(aa), _) if *aa == c_alias || *aa == s_alias => match *op {
                    Op::Xor => {
                        bad.insert(b.clone());
                        if *aa == c_alias {
                            bad.insert(rev_aliases.get(&s_alias).unwrap().clone());
                        } else {
                            bad.insert(rev_aliases.get(&c_alias).unwrap().clone());
                        }
                        aliases.insert(res.clone(), z_alias.clone());
                        rev_aliases.insert(z_alias.clone(), res.clone());
                    }
                    Op::And => {
                        bad.insert(b.clone());
                        if *aa == c_alias {
                            bad.insert(rev_aliases.get(&s_alias).unwrap().clone());
                        } else {
                            bad.insert(rev_aliases.get(&c_alias).unwrap().clone());
                        }
                        aliases.insert(res.clone(), a_alias.clone());
                        rev_aliases.insert(a_alias.clone(), res.clone());
                    }
                    Op::Or => {}
                },
                (_, Some(ba)) if *ba == c_alias || *ba == s_alias => match *op {
                    Op::Xor => {
                        bad.insert(a.clone());
                        if *ba == c_alias {
                            bad.insert(rev_aliases.get(&s_alias).unwrap().clone());
                        } else {
                            bad.insert(rev_aliases.get(&c_alias).unwrap().clone());
                        }
                        aliases.insert(res.clone(), z_alias.clone());
                        rev_aliases.insert(z_alias.clone(), res.clone());
                    }
                    Op::And => {
                        bad.insert(a.clone());
                        if *ba == c_alias {
                            bad.insert(rev_aliases.get(&s_alias).unwrap().clone());
                        } else {
                            bad.insert(rev_aliases.get(&c_alias).unwrap().clone());
                        }
                        aliases.insert(res.clone(), a_alias.clone());
                        rev_aliases.insert(a_alias.clone(), res.clone());
                    }
                    Op::Or => {}
                },
                _ => {}
            }
        }
        let c_alias = format!("c{:02}", i);
        for (res, (op, a, b)) in rules.iter() {
            match (aliases.get(a), aliases.get(b)) {
                (Some(aa), Some(ba))
                    if (*aa == a_alias && *ba == o_alias) || (*aa == o_alias && *ba == a_alias) =>
                {
                    match *op {
                        Op::Or => {
                            aliases.insert(res.clone(), c_alias.clone());
                            rev_aliases.insert(c_alias.clone(), res.clone());
                        }
                        Op::And | Op::Xor => {}
                    }
                }
                (Some(aa), _) if *aa == a_alias || *aa == o_alias => match *op {
                    Op::Or => {
                        bad.insert(b.clone());
                        if *aa == a_alias {
                            bad.insert(rev_aliases.get(&o_alias).unwrap().clone());
                        } else {
                            bad.insert(rev_aliases.get(&a_alias).unwrap().clone());
                        }
                        aliases.insert(res.clone(), c_alias.clone());
                        rev_aliases.insert(c_alias.clone(), res.clone());
                    }
                    Op::And | Op::Xor => {}
                },
                (_, Some(ba)) if *ba == a_alias || *ba == o_alias => match *op {
                    Op::Or => {
                        bad.insert(a.clone());
                        if *ba == a_alias {
                            bad.insert(rev_aliases.get(&o_alias).unwrap().clone());
                        } else {
                            bad.insert(rev_aliases.get(&a_alias).unwrap().clone());
                        }
                        aliases.insert(res.clone(), c_alias.clone());
                        rev_aliases.insert(c_alias.clone(), res.clone());
                    }
                    Op::And | Op::Xor => {}
                },
                _ => {}
            }
        }
    }
    let c_alias = format!("c{:02}", len - 1);
    let z_alias = format!("z{:02}", len);
    let most_significant_digit = rev_aliases.get(&c_alias).unwrap();
    if *most_significant_digit != z_alias {
        bad.insert(most_significant_digit.clone());
    }
    aliases.insert(most_significant_digit.to_string(), z_alias.clone());
    rev_aliases.insert(z_alias, most_significant_digit.to_string());
    // let aliased: HashSet<_> = aliases.keys().collect();
    // let ruled: HashSet<_> = rules.keys().collect();
    // let diff: HashSet<_> = ruled.difference(&aliased).collect();

    let mut bad: Vec<String> = bad.into_iter().collect();
    bad.sort();
    println!("{}", bad.join(","));
    // dbg!(diff);
    // dbg!(bad.len());
    // dbg!(bad);
    Ok(())
}
