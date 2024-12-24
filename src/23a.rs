// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use multimap::MultiMap;
use scan_rules::scanner::Word;
use std::{collections::HashSet, io::BufRead};
#[macro_use]
extern crate scan_rules;

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut graph = MultiMap::new();
    let mut edges = HashSet::new();
    for line in stdin.lock().lines() {
        let line = line?;
        let_scan!(line; (let a: Word<String>, "-", let b: Word<String>));
        graph.insert(a.clone(), b.clone());
        graph.insert(b.clone(), a.clone());
        edges.insert((a.clone(), b.clone()));
        // edges.insert((b.clone(), a.clone()));
    }

    let mut k3s = HashSet::new();
    for (a, b) in edges.iter() {
        let a_edges: HashSet<&String> = graph.get_vec(a).unwrap().iter().collect();
        let b_edges: HashSet<&String> = graph.get_vec(b).unwrap().iter().collect();
        for c in a_edges.intersection(&b_edges) {
            if a.starts_with("t") || b.starts_with("t") || c.starts_with("t") {
                let mut k3 = vec![a.to_string(), b.to_string(), c.to_string()];
                k3.sort();
                k3s.insert(k3);
            }
        }
    }

    println!("{}", k3s.len());
    // dbg!(k3s);

    Ok(())
}
