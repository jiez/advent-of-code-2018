#[macro_use] extern crate scan_fmt;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("/home/jie/projects/rust/advent/day6_1/input")
        .expect("Error cannot open the file");
    let mut contents = String::new();
    let mut places = Vec::new();

    file.read_to_string(&mut contents)
        .expect("Error reading input file");

    let mut max_x = 0;
    let mut max_y = 0;

    for line in contents.lines() {
        let (xx, yy) = scan_fmt!(line, "{}, {}", usize, usize);
        if let (Some(x), Some(y)) = (xx, yy) {
            places.push((x, y));
            if max_x < x { max_x = x; }
            if max_y < y { max_y = y; }
        }
    }

    let mut distances: Vec<Vec<usize>> = vec![vec![0; max_y + 1]; max_x + 1];

    for i in 0..max_x + 1 {
        for j in 0..max_y + 1 {
            for (x, y) in &places {
                if *x > i {
                    distances[i][j] += *x - i;
                } else {
                    distances[i][j] += i - *x;
                }
                if *y > j {
                    distances[i][j] += *y - j;
                } else {
                    distances[i][j] += j - *y;
                }
            }
        }
    }

    // if <= 10000 on boundary, we need to enlarge area
    for i in 0..max_x + 1 {
        if distances[i][0] <= 10000 {
            println!("({}, {}): {}", i, 0, distances[i][0]);
        }
        if distances[i][max_y] <= 10000 {
            println!("({}, {}): {}", i, max_y, distances[i][max_y]);
        }
    }

    for j in 0..max_y + 1 {
        if distances[0][j] <= 10000 {
            println!("({}, {}): {}", 0, j, distances[0][j]);
        }
        if distances[max_x][j] <= 10000 {
            println!("({}, {}): {}", max_x, j, distances[max_x][j]);
        }
    }

    let mut area = 0;
    for i in 0..max_x + 1 {
        for j in 0..max_y + 1 {
            if distances[i][j] < 10000 {
                area += 1;
            }
        }
    }


    println!("< 10000: {}", area);
}

