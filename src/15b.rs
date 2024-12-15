// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use itertools::Itertools;
use std::io::BufRead;

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

    pub fn scale_up(&mut self) {
        let mut new_tiles = vec![];
        for row in &self.tiles {
            let mut new_row = vec![];
            for c in row {
                let nc = match c {
                    b'#' => b"##",
                    b'O' => b"[]",
                    b'.' => b"..",
                    b'@' => b"@.",
                    _ => panic!("unexpected tile"),
                };
                new_row.extend_from_slice(nc);
            }
            new_tiles.push(new_row);
        }

        self.tiles = new_tiles;
        self.w = self.tiles[0].len();
    }

    pub fn push_stones(&mut self, pos_l: Point, dir: Dir) -> bool {
        // let pos_r = pos_l.step(Dir::Right);
        self.set2(&pos_l, b"..");
        let next_l = pos_l.step(dir);
        let next_l_val = self.get(&next_l).unwrap();
        let next_r = next_l.step(Dir::Right);
        let next_r_val = self.get(&next_r).unwrap();
        match &[next_l_val, next_r_val] {
            b".." => {
                self.set2(&next_l, b"[]");
                true
            }
            b"##" | b"#[" | b"]#" | b".#" | b"#." => {
                self.set2(&pos_l, b"[]");
                false
            }
            b"[]" => {
                if self.push_stones(next_l, dir) {
                    self.set2(&next_l, b"[]");
                    true
                } else {
                    self.set2(&pos_l, b"[]");
                    false
                }
            }
            b".[" => {
                if self.push_stones(next_r, dir) {
                    self.set2(&next_l, b"[]");
                    true
                } else {
                    self.set2(&pos_l, b"[]");
                    false
                }
            }
            b"]." => {
                if self.push_stones(next_l.step(Dir::Left), dir) {
                    self.set2(&next_l, b"[]");
                    true
                } else {
                    self.set2(&pos_l, b"[]");
                    false
                }
            }
            b"][" => {
                let backup = self.tiles.clone();
                if self.push_stones(next_l.step(Dir::Left), dir) && self.push_stones(next_r, dir) {
                    self.set2(&next_l, b"[]");
                    true
                } else {
                    self.tiles = backup;
                    self.set2(&pos_l, b"[]");
                    false
                }
            }
            tile => {
                panic!("invalid tile {}{}", tile[0] as char, tile[1] as char);
            }
        }
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
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut lines = stdin.lock().lines();
    let mut tiles = vec![];
    loop {
        let line = lines.next().unwrap()?;
        if line.is_empty() {
            break;
        }
        tiles.push(line.bytes().collect());
    }

    let mut moves = vec![];
    for line in lines {
        moves.extend(line?.bytes());
    }
    let moves = moves
        .into_iter()
        .map(|mov| match mov {
            b'^' => Dir::Up,
            b'v' => Dir::Down,
            b'<' => Dir::Left,
            b'>' => Dir::Right,
            _ => panic!("wrong move"),
        })
        .collect_vec();

    let mut map = Map::new(tiles);
    map.scale_up();
    let (start, _v) = map.all_points().find(|(_p, val)| *val == b'@').unwrap();
    map.set(&start, b'.');

    let mut cur = start;
    for mov in moves.into_iter() {
        let next = cur.step(mov);
        match map.get(&next).unwrap() {
            b'.' => {
                cur = next;
            }
            b'#' => {}
            b'[' => {
                if map.push_stones(next, mov) {
                    cur = next;
                }
            }
            b']' => {
                let stone_edge = next.step(Dir::Left);
                if map.push_stones(stone_edge, mov) {
                    cur = next;
                }
            }
            tile => {
                panic!("invalid tile {tile}");
            }
        }

        // println!("Move {}: {:?}", i, mov);
        // map.set(&cur, b'@');
        // map.print();
        // map.set(&cur, b'.');
        // println!();
        // println!();
    }
    let sum: i64 = map
        .all_points()
        .map(|(p, val)| if val == b'[' { 100 * p.y + p.x } else { 0 })
        .sum();

    println!("{sum}");
    Ok(())
}
