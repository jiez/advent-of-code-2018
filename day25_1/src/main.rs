#[macro_use] extern crate scan_fmt;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("/home/jie/projects/rust/advent/day25_1/input")
        .expect("Error cannot open the file");
    let mut contents = String::new();
    let mut fixed_points = Vec::new();
    let mut constellations: Vec<Vec<usize>> = Vec::new();

    file.read_to_string(&mut contents)
        .expect("Error reading input file");

    for line in contents.lines() {
        let (xx, yy, zz, rr) = scan_fmt!(line, "{},{},{},{}", i32, i32, i32, i32);
        if let (Some(x), Some(y), Some(z), Some(r)) = (xx, yy, zz, rr) {
            fixed_points.push(vec![x, y, z, r]);
        }
    }

    let num = fixed_points.len();

    println!("num: {}", num);
    let max_distance = 3;

    'outer: for i in 0..fixed_points.len() {
        let mut belongs_to = Vec::new();

        'middle: for j in 0..constellations.len() {
            let mut enough_close = false;
            for k in 0..constellations[j].len() {
                let l = constellations[j][k];
                if distance(&fixed_points[i], &fixed_points[l]) <= max_distance {
                    enough_close = true;
                    break;
                }
            }
            if enough_close {
                belongs_to.push(j);
            }
        }

        if belongs_to.len() == 0 {
            // not in range of any constellations
            let new_constellation = vec![i];
            constellations.push(new_constellation);
        } else if belongs_to.len() == 1 {
            constellations[belongs_to[0]].push(i);
        } else {
            let mut temp = vec![i];
            for m in 0..belongs_to.len() {
                temp.append(&mut constellations[belongs_to[m]]);
            }
            constellations.retain(|c| c.len() > 0);
            constellations.push(temp);
        }
    }

    println!("The number of constellations: {}", constellations.len());
}

#[inline(always)]
fn distance(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    let mut d;

    d = (a[0] - b[0]).abs();
    d += (a[1] - b[1]).abs();
    d += (a[2] - b[2]).abs();
    d += (a[3] - b[3]).abs();

    d
}
