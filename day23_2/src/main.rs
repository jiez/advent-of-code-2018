#[macro_use] extern crate scan_fmt;

use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut file = File::open("/home/jie/projects/rust/advent/day23_1/input")
        .expect("Error cannot open the file");
    let mut contents = String::new();
    let mut nanobots = Vec::new();

    file.read_to_string(&mut contents)
        .expect("Error reading input file");

    let mut min_x = std::i64::MAX;
    let mut min_y = std::i64::MAX;
    let mut min_z = std::i64::MAX;
    let mut max_x = std::i64::MIN;
    let mut max_y = std::i64::MIN;
    let mut max_z = std::i64::MIN;

    for line in contents.lines() {
        let (xx, yy, zz, rr) = scan_fmt!(line, "pos=<{},{},{}>, r={}", i64, i64, i64, i64);
        if let (Some(x), Some(y), Some(z), Some(r)) = (xx, yy, zz, rr) {
            nanobots.push((x, y, z, r));
            if min_x > x - r {
                min_x = x - r;
            }
            if min_y > y - r {
                min_y = y - r;
            }
            if min_z > z - r {
                min_z = z - r;
            }
            if max_x < x - r {
                max_x = x - r;
            }
            if max_y < y - r {
                max_y = y - r;
            }
            if max_z < z - r {
                max_z = z - r;
            }
        }
    }

    let num = nanobots.len();

    println!("num: {}, {} {} {}, {} {} {}", num, min_x, min_y, min_z, max_x, max_y, max_z);

    let mut max_num;
    let mut results_m: HashSet<(i64, i64, i64)> = HashSet::new();
    max_num = 0;
    max_num = in_range_of_most_num_of_nanobots(&nanobots, ((min_x - 999999) / 1000000, (min_y - 999999) / 1000000, (min_z - 999999) / 1000000), ((max_x + 999999)/ 1000000, (max_y + 999999) / 1000000, (max_z + 999999) / 1000000), 1000000, &mut results_m, max_num);
    println!("scale 1000000, max_num: {} num {}", max_num, results_m.len());
    for (a, b, c) in &results_m {
        println!("{} {} {}", a, b, c);
    }
    //results_m.insert((44, 43, 38));

    let mut results_10k: HashSet<(i64, i64, i64)> = HashSet::new();
    max_num = 0;
    for (x, y, z) in results_m {
        max_num = in_range_of_most_num_of_nanobots(&nanobots, (x * 100, y * 100, z * 100), ((x + 1) * 100, (y + 1) * 100, (z + 1) * 100), 10000, &mut results_10k, max_num);
    }
    println!("scale 10000, max_num: {} num {}", max_num, results_10k.len());

    let mut results_h: HashSet<(i64, i64, i64)> = HashSet::new();
    max_num = 0;
    for (x, y, z) in results_10k {
        max_num = in_range_of_most_num_of_nanobots(&nanobots, (x * 100, y * 100, z * 100), ((x + 1) * 100, (y + 1) * 100, (z + 1) * 100), 100, &mut results_h, max_num);
    }
    println!("scale 100, max_num: {} num {}", max_num, results_h.len());

    let mut results: HashSet<(i64, i64, i64)> = HashSet::new();
    max_num = 0;
    for (x, y, z) in results_h {
        max_num = in_range_of_most_num_of_nanobots(&nanobots, (x * 100, y * 100, z * 100), ((x + 1) * 100, (y + 1) * 100, (z + 1) * 100), 1, &mut results, max_num);
    }
    println!("scale 1, max_num: {} num {}", max_num, results.len());
    let mut shortest = std::i64::MAX;
    let mut shortest_x = 0;
    let mut shortest_y = 0;
    let mut shortest_z = 0;
    for pos in &results {
        if shortest > distance(&(pos.0, pos.1, pos.2, 0), &(0, 0, 0, 0)) {
            shortest = distance(&(pos.0, pos.1, pos.2, 0), &(0, 0, 0, 0));
            shortest_x = pos.0;
            shortest_y = pos.1;
            shortest_z = pos.2;
        }
    }
    println!("shortest {} ({}, {}, {})", shortest, shortest_x, shortest_y, shortest_z);
    let mut count = 0;
    for nb in &nanobots {
        if distance(nb, &(shortest_x, shortest_y, shortest_z, 0)) <= nb.3 {
            count += 1;
        }
    }
    println!("count {}", count);
}

fn in_range_of_most_num_of_nanobots(nanobots: &Vec<(i64, i64, i64, i64)>, min: (i64, i64, i64), max: (i64, i64, i64), scale: i64, results: &mut HashSet<(i64, i64, i64)>, max_num: u32) -> u32 {
    let mut most_count = max_num;
    let mut nanobots_scaled = Vec::new();

    println!("from ({} {} {}) to ({} {} {})", min.0, min.1, min.2, max.0, max.1, max.2);

    for nb in nanobots {
        //nanobots_scaled.push((nb.0/scale, nb.1/scale, nb.2/scale, (nb.3 + scale - 1)/scale));
        nanobots_scaled.push(scale_nanobot(&nb, scale));
    }

    for i in min.0..=max.0 {
        //println!("i = {}", i);
        for j in min.1..=max.1 {
            for k in min.2..=max.2 {
                let mut count = 0;
                for nb in &nanobots_scaled {
                    if distance(&(nb.0, nb.1, nb.2, 0), &(i, j, k, 0)) <= nb.3 {
                        count += 1;
                    }
                }
                if most_count == count {
                    results.insert((i, j, k));
                } else if most_count < count {
                    results.clear();
                    results.insert((i, j, k));
                    most_count = count;
                }
            }
        }
    }

    most_count
}

#[inline(always)]
fn scale_nanobot(a: &(i64, i64, i64, i64), s: i64) -> (i64, i64, i64, i64) {
    let mut b = (0, 0, 0, 0);
    if s == 1 {
        b = *a;
    } else {
    if a.0 < 0 {
        b.0 = (a.0 - s + 1) / s;
    } else {
        b.0 = a.0 / s;
    }
    if a.1 < 0 {
        b.1 = (a.1 - s + 1) / s;
    } else {
        b.1 = a.1 / s;
    }
    if a.2 < 0 {
        b.2 = (a.2 - s + 1) / s;
    } else {
        b.2 = a.2 / s;
    }
    b.3 = a.3 / s + 3;
    }

    b
}

#[inline(always)]
fn distance(a: &(i64, i64, i64, i64), b: &(i64, i64, i64, i64)) -> i64 {
    let mut d;

    d = (a.0 - b.0).abs();
    d += (a.1 - b.1).abs();
    d += (a.2 - b.2).abs();

    d
}
