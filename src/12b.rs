// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use std::collections::BTreeSet;
use std::io::BufRead;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    #[allow(dead_code)]
    fn clockwise_rotate(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    #[allow(dead_code)]
    fn counter_clockwise_rotate(&self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn neighbours(&self) -> [(Point, Dir); 4] {
        [
            (Self::new(self.x + 1, self.y), Dir::Right),
            (Self::new(self.x, self.y + 1), Dir::Up),
            (Self::new(self.x, self.y - 1), Dir::Down),
            (Self::new(self.x - 1, self.y), Dir::Left),
        ]
    }

    fn step(&self, dir: Dir) -> Point {
        match dir {
            Dir::Up => Self::new(self.x, self.y + 1),
            Dir::Right => Self::new(self.x + 1, self.y),
            Dir::Down => Self::new(self.x, self.y - 1),
            Dir::Left => Self::new(self.x - 1, self.y),
        }
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
        let mut perimeter = BTreeSet::new();
        let mut area: u64 = 0;

        seen.insert(p);
        stack.push(p);

        while let Some(p) = stack.pop() {
            let p_region = map.get(&p).unwrap();
            for (n, dir) in p.neighbours() {
                match (p_region, map.get(&n)) {
                    (_p_region, None) => {
                        perimeter.insert((p, dir));
                    }
                    (p_region, Some(n_region)) if p_region == n_region => {
                        if !seen.contains(&n) {
                            stack.push(n);
                            seen.insert(n);
                        }
                    }
                    (_p_region, Some(_n_region)) => {
                        perimeter.insert((p, dir));
                    }
                }
            }

            area += 1;
            visited.insert(p);
        }

        let mut sides: u64 = 0;
        while let Some(&(mut p, mut dir)) = perimeter.last() {
            loop {
                let walk_dir = dir.clockwise_rotate();
                let next = p.step(walk_dir);
                if perimeter.contains(&(next, dir)) {
                    perimeter.remove(&(next, dir));
                    p = next;
                    continue;
                }
                // Try turning clockwise
                let new_dir = dir.clockwise_rotate();
                if perimeter.contains(&(p, new_dir)) {
                    perimeter.remove(&(p, new_dir));
                    sides += 1;
                    dir = new_dir;
                    continue;
                }
                // Try turning counter-clockwise
                let new_dir = dir.counter_clockwise_rotate();
                let walk_dir = walk_dir.counter_clockwise_rotate();
                let next = next.step(walk_dir);
                if perimeter.contains(&(next, new_dir)) {
                    perimeter.remove(&(next, new_dir));
                    sides += 1;
                    dir = new_dir;
                    p = next;
                    continue;
                }
                break;
            }
        }
        // dbg!(p_val as char, area, sides);
        price += area * sides;
    }

    println!("{price}");
    Ok(())
}
