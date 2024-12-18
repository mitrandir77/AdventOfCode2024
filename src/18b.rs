// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use std::{
    collections::{BTreeSet, VecDeque},
    io::BufRead,
};
#[macro_use]
extern crate scan_rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn neighbours(&self) -> [Point; 4] {
        [
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y + 1),
            Self::new(self.x, self.y - 1),
            Self::new(self.x - 1, self.y),
        ]
    }
}

fn shortest_path(map: &BTreeSet<Point>, size: i64) -> u64 {
    // dbg!(&map, &size);
    let mut queue = VecDeque::new();
    queue.push_back((Point::new(0, 0), 0));
    let mut seen = BTreeSet::new();
    seen.insert(Point::new(0, 0));

    while let Some((p, dist)) = queue.pop_front() {
        if p.x == size && p.y == size {
            return dist;
        }
        for n in p.neighbours() {
            if p.x < 0 || p.x > size || p.y < 0 || p.y > size {
                continue;
            }
            if map.contains(&n) {
                continue;
            }
            if seen.insert(n) {
                queue.push_back((n, dist + 1));
            }
        }
    }
    u64::MAX
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut lines = stdin.lock().lines();
    let line = lines.next().unwrap()?;
    let_scan!(line; (let size: i64, let _iter: usize));

    let mut incoming_bytes = vec![];
    for line in lines {
        let line = line?;
        let_scan!(line; (let x: i64, ",", let y: i64));
        incoming_bytes.push(Point::new(x, y));
    }

    let mut map = BTreeSet::new();
    for p in incoming_bytes {
        map.insert(p);
        let path_len = shortest_path(&map, size);
        if path_len == u64::MAX {
            println!("{},{}", p.x, p.y);
            break;
        }
    }

    Ok(())
}
