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
use memoize::memoize;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum PadKind {
    NUMPAD,
    DIRPAD,
}

#[memoize]
fn shortest_paths(pad_kind: PadKind, start: u8, end: u8) -> Vec<Vec<u8>> {
    let map: &Map = match pad_kind {
        PadKind::NUMPAD => &NUMPAD,
        PadKind::DIRPAD => &DIRPAD,
    };
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

#[memoize]
fn inception_inner(start: u8, end: u8, pads: Vec<PadKind>) -> usize {
    let paths = shortest_paths(pads[0], start, end);

    paths
        .into_iter()
        .map(|path| inception(path, pads[1..].to_vec()))
        .min()
        .unwrap()
}

fn inception(input: Vec<u8>, pads: Vec<PadKind>) -> usize {
    if pads.is_empty() {
        return input.len();
    }
    let mut res = Default::default();
    for (s, e) in Some(b'A')
        .into_iter()
        .chain(input.iter().cloned())
        .tuple_windows()
    {
        let best = inception_inner(s, e, pads.clone());
        res += best;
    }
    res
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut res = 0;
    let mut pads = vec![PadKind::NUMPAD];
    for _i in 0..25 {
        pads.push(PadKind::DIRPAD);
    }
    for line in stdin.lock().lines() {
        let line = line?;
        let best = inception(line.as_bytes().to_vec(), pads.clone());

        let_scan!(line; (let num: usize, "A"));
        res += num * best;
    }
    println!("{res}");
    Ok(())
}
