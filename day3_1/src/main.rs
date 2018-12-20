#[macro_use] extern crate scan_fmt;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("/home/jie/projects/rust/advent/day3_1/input")
        .expect("Error cannot open the file");
    let mut contents = String::new();
    let mut fabric = vec![vec![0; 1000]; 1000];
    let mut sum = 0;

    file.read_to_string(&mut contents)
       .expect("Error reading input file");

    for line in contents.lines() {
       let (id, left, top, width, height) = scan_fmt!(line, "#{d} @ {d},{d}: {d}x{d}", usize, usize, usize, usize, usize);
       match (id, left, top, width, height) {
           (Some(_i), Some(l), Some(t), Some(w), Some(h)) => {
               for k in l..(l + w) {
                   for j in t..(t + h) {
                       fabric[k][j] = fabric[k][j] + 1;
                   }
               }
           }
           _ => println!("input error"),
       }
    }

    for k in 0..1000 {
        for j in 0..1000 {
            if fabric[k][j] > 1 {
                sum = sum + 1;
            }
        }
    }

    println!("{}", sum);
}
