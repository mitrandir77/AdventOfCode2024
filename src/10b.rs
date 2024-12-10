// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use std::collections::BTreeMap;
use std::io::BufRead;

use btreemultimap::BTreeMultiMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn neighbours(&self) -> [Point; 4] {
        [
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y + 1),
            Self::new(self.x, self.y - 1),
            Self::new(self.x - 1, self.y),
        ]
    }
}

struct Map {
    tiles: Vec<Vec<u8>>,
    h: usize,
    w: usize,
}

impl Map {
    fn new(tiles: Vec<Vec<u8>>) -> Self {
        let h = tiles.len();
        let w = tiles[0].len();
        Map { tiles, h, w }
    }

    fn get(&self, p: &Point) -> Option<u8> {
        let Ok(x) = usize::try_from(p.x) else {
            return None;
        };
        let Ok(y) = usize::try_from(p.y) else {
            return None;
        };
        let res = self.tiles.get(y).and_then(|row| row.get(x).copied());
        res
    }

    #[allow(dead_code)]
    fn contains(&self, p: &Point) -> bool {
        let Ok(x) = usize::try_from(p.x) else {
            return false;
        };
        let Ok(y) = usize::try_from(p.y) else {
            return false;
        };
        x < self.w && y < self.h
    }

    fn all_points(&self) -> impl Iterator<Item = (Point, u8)> + '_ {
        self.tiles.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, c)| (Point::new(x as i64, y as i64), *c))
        })
    }
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut tiles = vec![];
    for line in stdin.lock().lines() {
        let line = line?;
        tiles.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        );
    }
    let map = Map::new(tiles);

    let mut counters = BTreeMap::new();
    let mut by_height = BTreeMultiMap::new();
    for (p, height) in map.all_points() {
        let start_count = if height == 0 { 1 } else { 0 };
        counters.insert(p, start_count);
        by_height.insert(height, p);
    }

    for height in 1..=9 {
        if let Some(points) = by_height.get_vec(&height) {
            for p in points {
                let mut start_count = 0;
                for neighbour in p.neighbours() {
                    if map.get(&neighbour) == Some(height - 1) {
                        start_count += counters.get(&neighbour).unwrap();
                    }
                }
                counters.insert(*p, start_count);
                // dbg!((height, p, score));
            }
        }
    }

    let mut sum = 0;
    if let Some(points) = by_height.get_vec(&9) {
        for p in points {
            sum += counters.get(p).unwrap();
        }
    }

    println!("{sum}");
    Ok(())
}
