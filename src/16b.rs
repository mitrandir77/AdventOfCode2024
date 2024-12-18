// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use btreemultimap::BTreeMultiMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{collections::BTreeSet, io::BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn clockwise_rotate(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    pub fn counter_clockwise_rotate(&self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn neighbours(&self) -> [(Point, Dir); 4] {
        [
            (Self::new(self.x + 1, self.y), Dir::Right),
            (Self::new(self.x, self.y + 1), Dir::Up),
            (Self::new(self.x, self.y - 1), Dir::Down),
            (Self::new(self.x - 1, self.y), Dir::Left),
        ]
    }

    pub fn step(&self, dir: Dir) -> Point {
        match dir {
            Dir::Up => Self::new(self.x, self.y - 1),
            Dir::Right => Self::new(self.x + 1, self.y),
            Dir::Down => Self::new(self.x, self.y + 1),
            Dir::Left => Self::new(self.x - 1, self.y),
        }
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

    pub fn set2(&mut self, p: &Point, val: &[u8; 2]) {
        self.tiles[p.y as usize][p.x as usize] = val[0];
        self.tiles[p.y as usize][p.x as usize + 1] = val[1];
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

    pub fn tiles_on_shortest_path(&self, start: Point, end: Point) -> usize {
        let mut visited = BTreeMultiMap::new();
        // let mut trace = BTreeMultiMap::new();
        let mut queue = BinaryHeap::new();
        queue.push(State {
            pos: start,
            score: 0,
            dir: Dir::Right,
            src: (start, Dir::Right),
        });

        let mut best_score = None;

        while let Some(State {
            pos,
            dir,
            score,
            src,
        }) = queue.pop()
        {
            // dbg!(pos, dir, score);
            if let Some((_src, vis_score)) = visited.get(&(pos, dir)) {
                if *vis_score == score {
                    visited.insert((pos, dir), (src, score));
                }
                continue;
            }
            if pos == end && best_score.is_none() {
                best_score = Some(score);
            }
            if let Some(best_score) = best_score {
                if score > best_score {
                    continue;
                }
            }
            visited.insert((pos, dir), (src, score));

            match self.get(&pos).unwrap() {
                b'#' => continue,
                b'.' => (),
                b'S' => (),
                b'E' => (),
                tile => panic!("unexpected map tile '{}'", tile as char),
            }

            let next_dir = dir;
            let next_pos = pos.step(dir);
            queue.push(State {
                pos: next_pos,
                dir: next_dir,
                score: score + 1,
                src: (pos, dir),
            });

            let next_dir = dir.clockwise_rotate();
            queue.push(State {
                pos,
                dir: next_dir,
                score: score + 1000,
                src: (pos, dir),
            });

            let next_dir = dir.counter_clockwise_rotate();
            queue.push(State {
                pos,
                dir: next_dir,
                score: score + 1000,
                src: (pos, dir),
            });
        }

        let mut stack = vec![
            (end, Dir::Up),
            (end, Dir::Down),
            (end, Dir::Left),
            (end, Dir::Right),
        ];

        let mut on_path = BTreeSet::new();
        while let Some((pos, dir)) = stack.pop() {
            for (src, _score) in visited.get_vec(&(pos, dir)).unwrap_or(&Vec::new()) {
                on_path.insert(pos);
                if pos != start {
                    stack.push(*src);
                }
            }
        }
        on_path.len()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    score: usize,
    pos: Point,
    dir: Dir,
    src: (Point, Dir),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.dir.cmp(&other.dir))
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
    let (start, _v) = map.all_points().find(|(_p, val)| *val == b'S').unwrap();
    let (end, _v) = map.all_points().find(|(_p, val)| *val == b'E').unwrap();

    let score = map.tiles_on_shortest_path(start, end);

    println!("{score}");
    Ok(())
}
