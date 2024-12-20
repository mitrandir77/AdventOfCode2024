// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    io::BufRead,
};

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
pub struct Map {
    tiles: Vec<Vec<u8>>,
    h: usize,
    w: usize,
}

impl Map {
    pub fn new(tiles: Vec<Vec<u8>>) -> Self {
        let h = tiles.len();
        let w = tiles[0].len();
        Map { tiles, h, w }
    }

    pub fn get(&self, p: &Point) -> Option<u8> {
        let Ok(x) = usize::try_from(p.x) else {
            return None;
        };
        let Ok(y) = usize::try_from(p.y) else {
            return None;
        };
        let res = self.tiles.get(y).and_then(|row| row.get(x).copied());
        res
    }

    pub fn set(&mut self, p: &Point, val: u8) {
        self.tiles[p.y as usize][p.x as usize] = val
    }

    pub fn contains(&self, p: &Point) -> bool {
        let Ok(x) = usize::try_from(p.x) else {
            return false;
        };
        let Ok(y) = usize::try_from(p.y) else {
            return false;
        };
        x < self.w && y < self.h
    }

    pub fn all_points(&self) -> impl Iterator<Item = (Point, u8)> + '_ {
        self.tiles.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, c)| (Point::new(x as i64, y as i64), *c))
        })
    }

    pub fn print(&self) {
        for row in self.tiles.iter() {
            for c in row {
                let c = *c as char;
                print!("{c}");
            }
            println!();
        }
    }

    fn shortest_paths(&self, start: &Point) -> BTreeMap<Point, i64> {
        // dbg!(&map, &size);
        let mut queue = VecDeque::new();
        queue.push_back((*start, 0));
        let mut seen = BTreeSet::new();
        seen.insert(*start);

        let mut shortest_paths = BTreeMap::new();

        while let Some((p, dist)) = queue.pop_front() {
            shortest_paths.insert(p, dist);
            for n in p.neighbours() {
                match self.get(&n) {
                    None => {
                        continue;
                    }
                    Some(b'#') => {
                        continue;
                    }
                    _ => {
                        if seen.insert(n) {
                            queue.push_back((n, dist + 1));
                        }
                    }
                }
            }
        }
        shortest_paths
    }

    // returns map cheat_pos -> savings
    fn cheats(&self, shortest_paths: &BTreeMap<Point, i64>) -> BTreeMap<(Point, Point), i64> {
        let mut cheats = BTreeMap::new();

        for (p, _val) in self.all_points().filter(|(_p, v)| *v == b'#') {
            for s in p.neighbours() {
                for e in p.neighbours() {
                    if s == e {
                        continue;
                    }
                    if let (Some(sv), Some(ev)) = (self.get(&s), self.get(&e)) {
                        if sv == b'#' || ev == b'#' {
                            continue;
                        }
                        let savings =
                            shortest_paths.get(&e).unwrap() - shortest_paths.get(&s).unwrap() - 2;
                        if savings > 0 {
                            cheats.insert((s, e), savings);
                        }
                    }
                }
            }
        }

        cheats
    }
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut tiles = Vec::new();
    for line in stdin.lock().lines() {
        tiles.push(line?.as_bytes().to_vec());
    }
    let map = Map::new(tiles);
    let end = map.all_points().find(|(_p, val)| *val == b'E').unwrap().0;

    let shortest_paths = map.shortest_paths(&end);
    let cheats = map.cheats(&shortest_paths);

    let res = cheats.values().filter(|v| **v >= 100).count();

    println!("{res}");

    Ok(())
}
