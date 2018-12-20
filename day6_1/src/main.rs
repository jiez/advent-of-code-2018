#[macro_use] extern crate scan_fmt;

use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("/home/jie/projects/rust/advent/day6_1/input")
        .expect("Error cannot open the file");
    let mut contents = String::new();
    let mut boundaries = Vec::new();
    let mut all_new_boundaries = HashSet::new();

    file.read_to_string(&mut contents)
        .expect("Error reading input file");

    let mut max_x = 0;
    let mut max_y = 0;

    for line in contents.lines() {
        let (xx, yy) = scan_fmt!(line, "{}, {}", usize, usize);
        if let (Some(x), Some(y)) = (xx, yy) {
            let mut boundary = HashSet::new();
            boundary.insert((x, y));
            boundaries.push(boundary);
            if max_x < x { max_x = x; }
            if max_y < y { max_y = y; }
        }
    }

//    println!("max_x = {} max_y = {}", max_x, max_y);
    let mut area: Vec<Vec<i32>> = vec![vec![-1; max_y + 1]; max_x + 1];
    for (i, boundary) in boundaries.iter().enumerate() {
        for coordinate in boundary {
            let x = coordinate.0;
            let y = coordinate.1;
//            println!("{}, {}", x, y);
            area[x][y] = i as i32; // why dereference here???
        }
    }
/*
    for j in 0..max_y + 1 {
        for i in 0..max_x + 1 {
            if area[i][j] == -1 {
                print!(" -");
            } else if area[i][j] == -2 {
                print!(" +");
            } else {
                print!(" {}", area[i][j]);
            }
        }
        println!("");
    }
    println!("");
*/

    loop {
        all_new_boundaries.clear();
        for i in 0..boundaries.len() {
            let mut new_boundary = HashSet::new();
            for coordinate in boundaries[i].iter() {
                let x = coordinate.0 as i32;
                let y = coordinate.1 as i32;
                for (temp_x, temp_y) in [(x - 1, y), (x, y - 1), (x, y + 1), (x + 1, y)].iter() {
                    if *temp_x < 0 || *temp_y < 0 || *temp_x > max_x as i32 || *temp_y > max_y as i32 {
                        continue;
                    }
                    let x1 = *temp_x as usize;
                    let y1 = *temp_y as usize;

                    let ii = i as i32;

                    if ! all_new_boundaries.contains(&(x1, y1)) {
                        if area[x1][y1] == -1 {
                            all_new_boundaries.insert((x1, y1));
                            new_boundary.insert((x1, y1));
                            area[x1][y1] = ii;
                        }
                    } else if area[x1][y1] != ii {
                        new_boundary.insert((x1, y1));
                        if area[x1][y1] != -2 {
                            area[x1][y1] = -2;
                        }
                    }
                }
            }
            boundaries[i] = new_boundary;
        }
/*
        for j in 0..max_y + 1 {
            for i in 0..max_x + 1 {
                if area[i][j] == -1 {
                    print!(" -");
                } else if area[i][j] == -2 {
                    print!(" +");
                } else {
                    print!(" {}", area[i][j]);
                }
            }
            println!("");
        }
        println!("");
*/

        if all_new_boundaries.is_empty() {
            break;
        }
    }

    let mut bounded_areas = HashMap::new();
    
    for (i, boundary) in boundaries.iter().enumerate() {
        bounded_areas.insert(i, 0);
    }

    for i in 0..max_x + 1 {
        let mut place: usize;
        place = area[i][0] as usize;
        bounded_areas.remove(&place);
        place = area[i][max_y] as usize;
        bounded_areas.remove(&place);
    }
    for j in 0..max_y + 1 {
        let mut place: usize;
        place = area[0][j] as usize;
        bounded_areas.remove(&place);
        place = area[max_x][j] as usize;
        bounded_areas.remove(&place);
    }

    for i in 1..max_x {
        for j in 1..max_y {
            let place: usize;
            place = area[i][j] as usize;
            if bounded_areas.contains_key(&place) {
                *bounded_areas.get_mut(&place).unwrap() += 1;
            }
        }
    }

    let mut max = 0;
    for (place, a) in bounded_areas {
        if max < a {
            max = a;
        }
    }

    println!("{}", max);
}

