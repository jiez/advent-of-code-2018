#[macro_use] extern crate scan_fmt;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("/home/jie/projects/rust/advent/day10_1/input")
        .expect("Error cannot open the file");
    let mut contents = String::new();
    let mut points = Vec::new();
    let mut velocities = Vec::new();

    file.read_to_string(&mut contents)
        .expect("Error reading input file");


    for line in contents.lines() {
        let (xx, yy, vvx, vvy) = scan_fmt!(line, "position=<{},{}> velocity=<{},{}>", i32, i32, i32, i32);
        if let (Some(x), Some(y), Some(vx), Some(vy)) = (xx, yy, vvx, vvy) {
            points.push((x, y));
            velocities.push((vx, vy));
        }
    }

    let mut min_xsize = std::usize::MAX;
    let mut min_ysize = std::usize::MAX;

    for count in 0..20000 {
        let mut min_x = std::i32::MAX;
        let mut min_y = std::i32::MAX;
        let mut max_x = std::i32::MIN;
        let mut max_y = std::i32::MIN;

        for i in 0..points.len() {
            points[i].0 += velocities[i].0;
            points[i].1 += velocities[i].1;

            if min_x > points[i].0 { min_x = points[i].0; }
            if min_y > points[i].1 { min_y = points[i].1; }
            if max_x < points[i].0 { max_x = points[i].0; }
            if max_y < points[i].1 { max_y = points[i].1; }
        }

        let xsize: usize = (max_x - min_x + 1) as usize;
        let ysize: usize = (max_y - min_y + 1) as usize;

/*
        if min_xsize < 100 || min_ysize < 100 {
            println!("xsize {} ysize {} count {}", xsize, ysize, _count);
            break;
        } else {
           min_xsize = xsize;
           min_ysize = ysize;
        }
*/
//        println!("xsize {} ysize {}", xsize, ysize);
        if xsize < 70 && ysize < 20 {

        let mut sky = vec![vec![0; ysize]; xsize];


        for i in 0..points.len() {
            let x: usize = (points[i].0 - min_x) as usize;
            let y: usize = (points[i].1 - min_y) as usize;
            sky[x][y] = 1;
        }

        println!("count = {}", count);
        for j in 0..ysize {
            for i in 0..xsize {
                if sky[i][j] == 0 {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!("");
        }
        println!("");

        }
    }
}
