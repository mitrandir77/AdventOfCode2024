// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use itertools::Itertools;
use std::{
    collections::HashSet,
    io::BufRead,
    ops::{Add, Sub},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Point(usize, usize);
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Point {
    type Output = Option<Point>;
    fn sub(self, other: Point) -> Option<Point> {
        match (self.0.checked_sub(other.0), self.1.checked_sub(other.1)) {
            (Some(x), Some(y)) => Some(Point(x, y)),
            _ => None,
        }
    }
}

impl Point {
    fn mov(self, dir: Dir) -> Option<Point> {
        match dir {
            Dir::Up => self - Point(0, 1),
            Dir::Down => Some(self + Point(0, 1)),
            Dir::Left => self - Point(1, 0),
            Dir::Right => Some(self + Point(1, 0)),
        }
    }
}

// fn print_map(map: &[Vec<u8>]) {
//     for line in map.iter() {
//         let s = String::from_utf8_lossy(line);
//         eprintln!("{}", s);
//     }
// }
fn check(map: &mut [Vec<u8>]) -> bool {
    let mut pos = Point(0, 0);
    for (i, row) in map.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if *tile == b'^' {
                pos = Point(j, i);
            }
        }
    }

    let mut dir = Dir::Up;

    let mut vis = HashSet::new();
    let h = map.len();
    let w = map[0].len();
    loop {
        // dbg!(&pos, &dir);
        map[pos.1][pos.0] = b'X'; // visited
        // print_map(map);
        vis.insert((pos, dir));
        let new_pos = if let Some(new_pos) = pos.mov(dir) {
            if new_pos.0 >= h || new_pos.1 >= w {
                return false;
            }
            new_pos
        } else {
            return false;
        };
        // dbg!(&new_pos);
        match map[new_pos.1][new_pos.0] {
            b'X' => {
                if vis.contains(&(new_pos, dir)) {
                    return true;
                }
                pos = new_pos;
            }
            b'.' => {
                pos = new_pos;
            }
            _ => {
                dir = dir.turn_right();
                continue;
            }
        }
    }
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut map = vec![];
    for line in stdin.lock().lines() {
        let line = line?;
        map.push(line.as_bytes().iter().cloned().collect_vec());
    }

    let h = map.len();
    let w = map[0].len();
    let mut sum = 0;
    for x in 0..w {
        for y in 0..h {
            if map[y][x] != b'.' {
                continue;
            }
            let mut new_map = map.clone();
            new_map[y][x] = b'#';
            if check(&mut new_map) {
                sum += 1;
            }
        }
    }

    println!("{sum}");

    Ok(())
}
