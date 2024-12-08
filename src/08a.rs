// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
use btreemultimap::BTreeMultiMap;
use std::{collections::BTreeSet, io::BufRead};

#[allow(dead_code)]
fn print_map(map: &[Vec<u8>], antinodes: &BTreeSet<(usize, usize)>) {
    let mut map = map.to_vec();

    for (y, row) in map.iter_mut().enumerate() {
        for (x, c) in row.iter_mut().enumerate() {
            if antinodes.contains(&(y, x)) {
                *c = b'#';
            }
        }
    }

    for line in map.iter() {
        let s = String::from_utf8_lossy(line);
        eprintln!("{}", s);
    }
}

fn count_antinodes(map: &mut [Vec<u8>]) -> usize {
    let mut antinodes = BTreeSet::new();
    let mut frequency_map = BTreeMultiMap::new();
    let h = map.len();
    let w = map[0].len();

    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c != b'.' {
                frequency_map.insert(*c, (y, x));
            }
        }
    }

    for freq in frequency_map.keys() {
        // dbg!(*freq as char);
        for (ay, ax) in frequency_map.get_vec(freq).unwrap() {
            for (by, bx) in frequency_map.get_vec(freq).unwrap() {
                if ay == by && ax == bx {
                    continue;
                }
                let (xd, yd) = (ax - bx, ay - by);

                let (ny, nx) = (ay + yd, ax + xd);
                if (0..h).contains(&ny) && (0..w).contains(&nx) {
                    antinodes.insert((ny, nx));
                }
            }
        }
    }
    // print_map(map, &antinodes);

    antinodes.len()
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut map = vec![];
    for line in stdin.lock().lines() {
        let line = line?;
        map.push(line.as_bytes().to_vec());
    }
    let sum = count_antinodes(&mut map);
    // print_map(&map);
    println!("{sum}");

    Ok(())
}
