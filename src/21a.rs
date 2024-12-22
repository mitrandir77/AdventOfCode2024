// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::{collections::VecDeque, io::BufRead};
#[macro_use]
extern crate scan_rules;

pub mod map;
use map::*;

fn shortest_paths(map: &Map, start: u8, end: u8) -> Vec<Vec<u8>> {
    // dbg!(start as char, end as char);
    let mut res = vec![];
    let start = map.all_points().find(|(_p, v)| *v == start).unwrap().0;
    let end = map.all_points().find(|(_p, v)| *v == end).unwrap().0;

    let mut queue = VecDeque::new();
    let mut min_dist = None;
    queue.push_back((start, 0, vec![]));

    while let Some((p, dist, dirs)) = queue.pop_front() {
        if let Some(min_dist) = min_dist {
            if dist > min_dist {
                break;
            }
        }
        if p == end {
            min_dist = Some(dist);
            res.push(encode(&dirs));
        }
        for (np, nd) in p.neighbours() {
            if let Some(nval) = map.get(&np) {
                if nval == b' ' {
                    continue;
                }
                let mut ndirs = dirs.clone();
                ndirs.push(nd);
                queue.push_back((np, dist + 1, ndirs));
            }
        }
    }
    res
}

fn encode(dirs: &[Dir]) -> Vec<u8> {
    dirs.iter()
        .map(|d| match d {
            Dir::Up => b'^',
            Dir::Down => b'v',
            Dir::Left => b'<',
            Dir::Right => b'>',
        })
        .chain(Some(b'A'))
        .collect()
}

const NUMPAD_STR: &str = "\
789
456
123
 0A";
const DIRPAD_STR: &str = " ^A
<v>";

lazy_static! {
    static ref NUMPAD: Map = Map::new(
        NUMPAD_STR
            .lines()
            .map(|l| l.bytes().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );
    static ref DIRPAD: Map = Map::new(
        DIRPAD_STR
            .lines()
            .map(|l| l.bytes().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );
}

fn inception(input: &[u8], pads: &[&Map]) -> Vec<u8> {
    if pads.is_empty() {
        return input.to_vec();
    }
    let mut res = vec![];
    for (s, e) in Some(b'A')
        .into_iter()
        .chain(input.iter().cloned())
        .tuple_windows()
    {
        let paths = shortest_paths(pads[0], s, e);
        let best = paths
            .iter()
            .map(|path| inception(path, &pads[1..]))
            .min_by(|a, b| a.len().cmp(&b.len()))
            .unwrap();
        res.extend(best);
    }
    res
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut res = 0;
    for line in stdin.lock().lines() {
        let line = line?;
        let best = inception(line.as_bytes(), &[&NUMPAD, &DIRPAD, &DIRPAD]);

        let_scan!(line; (let num: usize, "A"));
        res += num * best.len();
    }
    println!("{res}");
    Ok(())
}
