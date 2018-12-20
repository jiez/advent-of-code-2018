#[macro_use] extern crate scan_fmt;

use std::fs::File;
use std::io::Read;

// maybe we can use OO here

struct Ground {
    square_meters: Vec<Vec<u32>>,
    x_min: usize,
    y_min: usize,
    x_size: usize,
    y_size: usize,
}

enum Side {
    Open,
    Closed(usize),
}

fn main() {
    let mut file = File::open("/home/jie/projects/rust/advent/day17_1/input")
        .expect("Error cannot open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading input file");

    let spring_x = 500;
    let spring_y = 0;

    let mut ground_x_min = spring_x;
    let mut ground_x_max = spring_x;
    let mut ground_y_min = std::usize::MAX;
    let mut ground_y_max = spring_y;
    for line in contents.lines() {
        let (xx, yy0, yy1) = scan_fmt!(line, "x={d}, y={d}..{d}", usize, usize, usize);
        if let (Some(x), Some(y0), Some(y1)) = (xx, yy0, yy1) {
            if x < ground_x_min {
                ground_x_min = x;
            }
            if x > ground_x_max {
                ground_x_max = x;
            }
            if y0 < ground_y_min {
                ground_y_min = y0;
            }
            if y1 > ground_y_max {
                ground_y_max = y1;
            }
        }
        let (yy, xx0, xx1) = scan_fmt!(line, "y={d}, x={d}..{d}", usize, usize, usize);
        if let (Some(y), Some(x0), Some(x1)) = (yy, xx0, xx1) {
            if x0 < ground_x_min {
                ground_x_min = x0;
            }
            if x1 > ground_x_max {
                ground_x_max = x1;
            }
            if y < ground_y_min {
                ground_y_min = y;
            }
            if y > ground_y_max {
                ground_y_max = y;
            }
        }
    }

    println!("ground size x:{}..{}, y:{}..{}", ground_x_min, ground_x_max, ground_y_min, ground_y_max);

    if ground_x_min > 0 {
        ground_x_min -= 1;
    }
    ground_x_max += 1;

    let ground_size_x = ground_x_max - ground_x_min + 1;
    let ground_size_y = ground_y_max - spring_y + 1;

    let mut ground = Ground{square_meters: vec![vec![0; ground_size_y]; ground_size_x], x_min: ground_x_min, y_min: spring_y, x_size: ground_size_x, y_size: ground_size_y};

    ground.square_meters[spring_x - ground_x_min][spring_y - spring_y] = 1;

    for line in contents.lines() {
        let (xx, yy0, yy1) = scan_fmt!(line, "x={d}, y={d}..{d}", usize, usize, usize);
        if let (Some(x), Some(y0), Some(y1)) = (xx, yy0, yy1) {
            for j in y0..=y1 {
                ground.square_meters[x - ground_x_min][j - spring_y] = 2;
            }
        }
        let (yy, xx0, xx1) = scan_fmt!(line, "y={d}, x={d}..{d}", usize, usize, usize);
        if let (Some(y), Some(x0), Some(x1)) = (yy, xx0, xx1) {
            for i in x0..=x1 {
                ground.square_meters[i - ground_x_min][y - spring_y] = 2;
            }
        }
    }

    print_ground(&ground);

    fill(&mut ground, spring_x - ground_x_min, spring_y - spring_y);

    print_ground(&ground);

    let mut tiles = 0;
    let mut retains = 0;
    for i in 0..ground.x_size {
        for j in ground_y_min..ground.y_size {
            if ground.square_meters[i][j] == 3 || ground.square_meters[i][j] == 4 {
                tiles += 1;
            }
            if ground.square_meters[i][j] == 4 {
                retains += 1;
            }
        }
    }

    println!("water passed tiles: {}", tiles);
    println!("retained water: {}", retains);
}

fn fill(ground: &mut Ground, x: usize, y: usize) {
    if ground.square_meters[x][y] != 1 {
        ground.square_meters[x][y] = 3;
    }

    if ! in_ground(ground, x, y + 1) {
        return;
    }

    if leaking(ground, x, y + 1) {
        fill(ground, x, y + 1);
    }

    if ! leaking(ground, x, y + 1) {
        let mut left_side = Side::Open;
        let mut right_side = Side::Open;

        if in_ground(ground, x - 1, y) {
            if is_clay(ground, x - 1, y) {
                left_side = Side::Closed(x);
            } else {
                left_side = flow_left(ground, x - 1, y);
            }
        }

        if in_ground(ground, x + 1, y) {
            if is_clay(ground, x + 1, y) {
                right_side = Side::Closed(x);
            } else {
                right_side = flow_right(ground, x + 1, y);
            }
        }

        if let Side::Closed(x_left) = left_side {
            if let Side::Closed(x_right) = right_side {
                for i in x_left..=x_right {
                    ground.square_meters[i][y] = 4;
                }
            }
        }
    }
}


fn flow_left(ground: &mut Ground, x0: usize, y: usize) -> Side {
    let mut x = x0;
    loop {
        ground.square_meters[x][y] = 3;

        if leaking(ground, x, y + 1) {
            fill(ground, x, y + 1);
        }
        if ! leaking(ground, x, y + 1) {
            if is_clay(ground, x - 1, y) {
                return Side::Closed(x);
            } else {
                x -= 1;
            }
        } else {
            return Side::Open;
        }
    }
}

fn flow_right(ground: &mut Ground, x0: usize, y: usize) -> Side {
    let mut x = x0;
    loop {
        ground.square_meters[x][y] = 3;

        if leaking(ground, x, y + 1) {
            fill(ground, x, y + 1);
        }
        if ! leaking(ground, x, y + 1) {
            if is_clay(ground, x + 1, y) {
                return Side::Closed(x);
            } else {
                x += 1;
            }
        } else {
            return Side::Open;
        }
    }
}

fn leaking(ground: &Ground, x: usize, y: usize) -> bool {
    if ground.square_meters[x][y] == 2 || ground.square_meters[x][y] == 4 { false } else { true }
}

fn in_ground(ground: &Ground, x: usize, y: usize) -> bool {
    if x < ground.x_size && y < ground.y_size { true } else { false }
}

fn is_clay(ground: &Ground, x: usize, y: usize) -> bool {
    if ground.square_meters[x][y] == 2 { true } else { false }
}
/*
           x
           |
           v
       ....o....
       ....o....
   y ->....o....
       ....o....

   ground[x][y]
*/
fn print_ground(ground: &Ground) {
    for n in 0..ground.y_size {
        for m in 0..ground.x_size {
            match ground.square_meters[m][n] {
            0 => print!("."),
            1 => print!("+"),
            2 => print!("#"),
            3 => print!("|"),
            4 => print!("~"),
            _ => ()
            }
        }
        println!("");
    }
    println!("");
}
/* Algorithm:

   fill(x, y): fill starting from (x, y)

   fill(500, 0);

   What does fill(x, y) do:

   if ground[y][x] != '+' { ground[y][x] = '|' }

   if (x, y + 1) out of map { return }

   if (x, y + 1) leaking {
       fill(x, y + 1);
   }

   if (x, y + 1) not leaking {
       if (x - 1, y) is still on map {
           if (x - 1, y) is clay {
               left_side = closed(x);
           } else {
               left_side = flow_left(x - 1, y);
           }
       }
       if (x + 1, y) is still on map {
           if (x + 1, y) is clay {
               right_side = closed(x);
           } else {
               right_side = flow_right(x + 1, y);
           }
       }

       if left_side is closed(x_left) && right_side is closed(x_right) {
           fill water from (x_left, y) to (x_right, y);
       }
   }

   What does flow_left(x, y) do:

   loop {
       ground[y][x] = '|'

       if (x, y + 1) leaking {
           fill(x, y + 1)
       }
       if (x, y + 1) not leaking {
           if (x - 1, y) is clay {
               return closed(x);
           } else {
               x <- x - 1
           }
       } else {
           return open;
       }
   }
*/

