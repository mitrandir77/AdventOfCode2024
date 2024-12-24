// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use scan_rules::scanner::Word;
use std::{
    collections::{BTreeMap, BTreeSet},
    io::BufRead,
};
#[macro_use]
extern crate scan_rules;

type Node = (u8, u8);
type Graph = BTreeMap<Node, BTreeSet<Node>>;

fn extend_clique(
    graph: &Graph,
    k: BTreeSet<Node>,
    mut candidates: BTreeSet<Node>,
    max: &mut BTreeSet<Node>,
) {
    for cand in candidates.clone().iter() {
        let cand_edges = graph.get(cand).unwrap();
        if cand_edges.intersection(&k).count() == k.len() {
            let mut k = k.clone();
            candidates.remove(cand);
            k.insert(*cand);
            if k.len() > max.len() {
                *max = k.clone();
            }
            extend_clique(graph, k, candidates.clone(), max);
        }
    }
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut graph = BTreeMap::<Node, BTreeSet<Node>>::new();
    for line in stdin.lock().lines() {
        let line = line?;
        let_scan!(line; (let a: Word<String>, "-", let b: Word<String>));
        let a = (
            a.chars().next().unwrap() as u8,
            a.chars().nth(1).unwrap() as u8,
        );
        let b = (
            b.chars().next().unwrap() as u8,
            b.chars().nth(1).unwrap() as u8,
        );
        graph.entry(a).or_default().insert(b);
        graph.entry(b).or_default().insert(a);
    }

    let mut max = BTreeSet::new();
    let cand = graph.keys().cloned().collect();
    extend_clique(&graph, BTreeSet::new(), cand, &mut max);
    let mut max: Vec<String> = max
        .into_iter()
        .map(|(a, b)| format!("{}{}", a as char, b as char))
        .collect();
    max.sort();
    println!("{}", max.join(","));

    Ok(())
}
