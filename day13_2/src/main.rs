use std::fs::File;
use std::io::Read;
use std::cmp::Ordering;

struct Cart {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
    turn: u32,
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        if self.y < other.y {
            Ordering::Less
        } else if self.x < other.x {
            Ordering::Less
        } else if self.x == other.x && self.y == other.y {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Cart) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Cart {}


fn main() {
    let mut file = File::open("/home/jie/projects/rust/advent/day13_1/input")
        .expect("Error cannot open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading input file");

    let mut map_size_x = 0;
    let mut map_size_y = 0;
    for line in contents.lines() {
        map_size_x += 1;
        if map_size_y < line.len() {
            map_size_y = line.len();
        }
    }

    println!("map size {}x{}", map_size_x, map_size_y);

    let mut map: Vec<Vec<u32>> = vec![vec![0; map_size_y]; map_size_x];

    let mut carts: Vec<Cart> = Vec::new();

    let mut i = 0;
    for line in contents.lines() {
        for j in 0..line.len() {
            println!("{}, {}", i, j);
            match &line[j..j+1] {
            " " => map[i][j] = 0,
            "-" => map[i][j] = 1,
            "|" => map[i][j] = 2,
            "\\" => map[i][j] = 3,
            "/" => map[i][j] = 4,
            "+" => map[i][j] = 5,

            // assume no carts on cross

            // we use 0x10 to mark the cart being at this place
            ">" => {
                map[i][j] = 1 + 0x10;
                let c: Cart = Cart{ x: i as i32, y: j as i32, vx: 0, vy: 1, turn: 0};
                carts.push(c);
            }
            "<" => {
                map[i][j] = 1 + 0x10;
                let c: Cart = Cart{ x: i as i32, y: j as i32, vx: 0, vy: -1, turn: 0};
                carts.push(c);
            }
            "^" => {
                map[i][j] = 2 + 0x10;
                let c: Cart = Cart{ x: i as i32, y: j as i32, vx: -1, vy: 0, turn: 0};
                carts.push(c);
            }
            "v" => {
                map[i][j] = 2 + 0x10;
                let c: Cart = Cart{ x: i as i32, y: j as i32, vx: 1, vy: 0, turn: 0};
                carts.push(c);
            }
            _ => ()
            }
        }
        i += 1;
    }

    println!("carts:");
    for c in &carts {
        println!("({}, {}) v: ({}, {}) turn: {}", c.x, c.y, c.vx, c.vy, c.turn);
    }

    'outer: loop {
        for c in &mut carts {

            // if the cart has been removed
            if map[c.x as usize][c.y as usize] < 0x10 {
                continue;
            }

            // move the cart out of current place
            map[c.x as usize][c.y as usize] &= 0xf;

            print!("({}, {}) ({}, {}) turn: {} ==> ", c.x, c.y, c.vx, c.vy, c.turn);

            c.x += c.vx;
            c.y += c.vy;

/* Borrow Checker does not like this
            match carts.binary_search_by(|cart| cart.cmp(&build_cart(c.x, c.y, 0, 0, 0))) {
                Ok(_) => {
                    println!("crash point: ({}, {})", c.x, c.y);
                    break 'outer;
                }
                _ => ()
            }
*/

            // decide new v
            match map[c.x as usize][c.y as usize] {
                1 | 2 => (),
                3 => {
                    let mut new_vx = c.vy;
                    let mut new_vy = c.vx;
                    c.vx = new_vx;
                    c.vy = new_vy;
                }
                4 => {
                    let mut new_vx = - c.vy;
                    let mut new_vy = - c.vx;
                    c.vx = new_vx;
                    c.vy = new_vy;
                }
                5 => {
                    c.turn += 1;
                    match c.turn % 3 {
                        1 => {
                            // turn left
                            //  0 -1
                            //  1  0
                            let mut new_vx = - c.vy;
                            let mut new_vy = c.vx;
                            c.vx = new_vx;
                            c.vy = new_vy;
                        }
                        2 => (), // go straight
                        0 => {
                            // turn right 
                            //   0  1
                            //  -1  0
                            let mut new_vx = c.vy;
                            let mut new_vy = - c.vx;
                            c.vx = new_vx;
                            c.vy = new_vy;
                        }
                        _ => (),
                    }
                }
                0x11...0x1f => {
                    println!("crash point: ({}, {})", c.x, c.y);
                    map[c.x as usize][c.y as usize] &= 0xf;
                    continue;
                }
                _ => ()
            }


            println!("({}, {}) ({}, {}) turn: {}", c.x, c.y, c.vx, c.vy, c.turn);
            map[c.x as usize][c.y as usize] |= 0x10;
        }

        // remove to_remove carts
        carts.retain(|ref c| map[c.x as usize][c.y as usize] > 0x10);

        println!("carts:");
        for c in &carts {
            println!("({}, {}) v: ({}, {}) turn: {} @{}", c.x, c.y, c.vx, c.vy, c.turn, map[c.x as usize][c.y as usize]);
        }


        if carts.len() == 1 {
            println!("last cart @ ({}, {})", carts[0].x, carts[0].y);
            println!("Answer ({}, {})", carts[0].y, carts[0].x);
            break;
        }

        carts.sort();
    }
}
