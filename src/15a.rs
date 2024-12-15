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

    pub fn push_stones(&mut self, pos: Point, dir: Dir) -> bool {
        let next = pos.step(dir);
        let next_val = self.get(&next).unwrap();
        match next_val {
            b'.' => {
                self.set(&next, b'O');
                self.set(&pos, b'.');
                true
            }
            b'#' => false,
            b'O' => {
                if self.push_stones(next, dir) {
                    self.set(&next, b'O');
                    self.set(&pos, b'.');
                    true
                } else {
                    false
                }
            }
            tile => {
                panic!("invalid tile {tile}");
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
            b'O' => {
                if map.push_stones(next, mov) {
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
        .map(|(p, val)| if val == b'O' { 100 * p.y + p.x } else { 0 })
        .sum();

    println!("{sum}");
    Ok(())
}
