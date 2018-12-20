#[macro_use] extern crate scan_fmt;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

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

    let mut area: Vec<Vec<i32>> = vec![vec![-1; max_y + 1]; max_x + 1];

    for i in 0..max_x + 1 {
        for j in 0..max_y + 1 {
            let mut min = max_x + max_y + 1;
            let mut min_id: i32 = 0;
            for (id, (x, y)) in places.iter().enumerate() {
                let mut d = 0;
                if *x > i {
                    d += *x - i;
                } else {
                    d += i - *x;
                }
                if *y > j {
                    d += *y - j;
                } else {
                    d += j - *y;
                }

                if d < min {
                    min = d;
                    min_id = id as i32;
                } else if d == min {
                    min_id = -1;
                }

                area[i][j] = min_id;
            }
        }
    }

    let mut bounded_areas = HashMap::new();
    
    for (i, _) in places.iter().enumerate() {
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
    for (_, a) in bounded_areas {
        if max < a {
            max = a;
        }
    }

    println!("{}", max);

}

