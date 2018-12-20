#[macro_use] extern crate scan_fmt;

use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut file = File::open("/home/jie/projects/rust/advent/day23_1/input")
        .expect("Error cannot open the file");
    let mut contents = String::new();
    let mut nanobots = HashSet::new();

    file.read_to_string(&mut contents)
        .expect("Error reading input file");

    let mut strongest = (0, 0, 0, 0);
    let mut min_r = std::u64::MAX;
    let mut min_x = std::i64::MAX;
    let mut min_y = std::i64::MAX;
    let mut min_z = std::i64::MAX;
    let mut max_x = std::i64::MIN;
    let mut max_y = std::i64::MIN;
    let mut max_z = std::i64::MIN;
    for line in contents.lines() {
        let (xx, yy, zz, rr) = scan_fmt!(line, "pos=<{},{},{}>, r={}", i64, i64, i64, u64);
        if let (Some(x), Some(y), Some(z), Some(r)) = (xx, yy, zz, rr) {
            nanobots.insert((x, y, z, r));
            if strongest.3 < r {
                strongest = (x, y, z, r);
            }
            if min_r > r {
                min_r = r;
            }
            if min_x > x {
                min_x = x;
            }
            if min_y > y {
                min_y = y;
            }
            if min_z > z {
                min_z = z;
            }
            if max_x < x {
                max_x = x;
            }
            if max_y < y {
                max_y = y;
            }
            if max_z < z {
                max_z = z;
            }
        }
    }

    nanobots.retain(|nb| distance(&nb, &strongest) <= strongest.3);
    let num = nanobots.len();

    println!("The strongest nanobot has r {}, contains {} nanobots. The weakest has r {}.", strongest.3, num, min_r);
    println!("nanobots from ({}, {}, {}) to ({}, {}, {})", min_x, min_y, min_z, max_x, max_y, max_z);
}

fn distance(a: &(i64, i64, i64, u64), b: &(i64, i64, i64, u64)) -> u64 {
    let mut d = 0;

    if a.0 >= b.0 {
        d += a.0 - b.0;
    } else {
        d += b.0 - a.0;
    }
    if a.1 >= b.1 {
        d += a.1 - b.1;
    } else {
        d += b.1 - a.1;
    }
    if a.2 >= b.2 {
        d += a.2 - b.2;
    } else {
        d += b.2 - a.2;
    }

    d as u64
}
