// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use std::collections::BTreeSet;
use std::io::BufRead;

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
        tiles.push(line.bytes().collect());
    }
    let map = Map::new(tiles);

    let mut price: u64 = 0;
    let mut visited = BTreeSet::new();
    let mut seen = BTreeSet::new();
    let mut stack = Vec::new();
    for (p, _p_val) in map.all_points() {
        if visited.contains(&p) {
            continue;
        }
        let mut perimeter: u64 = 0;
        let mut area: u64 = 0;

        seen.insert(p);
        stack.push(p);

        while let Some(p) = stack.pop() {
            let p_region = map.get(&p).unwrap();
            for n in p.neighbours() {
                match (p_region, map.get(&n)) {
                    (_p_region, None) => {
                        perimeter += 1;
                    }
                    (p_region, Some(n_region)) if p_region == n_region => {
                        if !seen.contains(&n) {
                            stack.push(n);
                            seen.insert(n);
                        }
                    }
                    (_p_region, Some(_n_region)) => {
                        perimeter += 1;
                    }
                }
            }
            area += 1;
            visited.insert(p);
        }
        price += area * perimeter;
    }

    println!("{price}");
    Ok(())
}
