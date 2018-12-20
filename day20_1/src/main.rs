use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

type Map = Vec<Vec<u32>>;
/*
           x
           |
           v
       ....o....
       ....o....
   y ->....o....
       ....o....

   map[x][y]
*/

fn main() {
    let mut file = File::open("/home/jie/projects/rust/advent/day20_1/input")
        .expect("Error cannot open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading input file");

    //let contents = String::from("^ENWWW(NEEE|SSE(EE|N))$");
    //let contents = String::from("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$");
    //let contents = String::from("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$");
    //let contents = String::from("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$");
    let routes = contents.into_bytes();

    let map_max_size = routes.len() * 2 + 1;
    let orig_x = routes.len();
    let orig_y = routes.len();
    //let map_max_size = routes.len() / 4 * 2 + 1;
    //let orig_x = routes.len() / 4;
    //let orig_y = routes.len() / 4;

    println!("map max size: {}, origin point: ({}, {})", map_max_size, orig_x, orig_y);

    // 0: '.'
    // 1: '|'
    // 2: '-'
    // 3: '#'

    // all walls at beginning
    let mut map: Map = vec![vec![3; map_max_size]; map_max_size];
    map[orig_x][orig_y] = 0;

    let mut current_heads = HashSet::new();
    let mut level_heads = HashSet::new();
    let mut heads_stack = Vec::new();
    let mut min_x = orig_x;
    let mut max_x = orig_x;
    let mut min_y = orig_y;
    let mut max_y = orig_y;

    current_heads.insert((orig_x, orig_y));
    let mut index = 1;

    loop {
        //println!("{}@{}", routes[index] as char, index);
        match routes[index] as char {
            'N' | 'S' | 'W' | 'E' => {
                let mut temp_heads = HashSet::new();
                for head in &current_heads {
                    //println!("Head: ({}, {})", head.0, head.1);
                    let (a, b) = mark_map(&mut map, head.0, head.1, routes[index]);
                    if a < min_x { min_x = a; }
                    if a > max_x { max_x = a; }
                    if b < min_y { min_y = b; }
                    if b > max_y { max_y = b; }
                    temp_heads.insert((a, b));

                }
                current_heads = temp_heads;
                //print_map(&map, orig_x, orig_y, min_x, max_x, min_y, max_y, &level_heads, &current_heads);
            }

            '(' => {
                heads_stack.push((current_heads.clone(), level_heads.clone()));
                level_heads.clear();
            }

            '|' => {
                for head in &current_heads {
                    level_heads.insert(*head);
                }
                current_heads.clear();
                current_heads.clone_from(&heads_stack[heads_stack.len() - 1].0);
            }

            ')' => {
                for head in &level_heads {
                    current_heads.insert(*head);
                }
                level_heads.clear();
                level_heads.clone_from(&heads_stack[heads_stack.len() - 1].1);
                heads_stack.pop();
            }

            '$' => { break; }

            _ => (),
        }

        index += 1;
    }

    //print_map(&map, orig_x, orig_y, min_x, max_x, min_y, max_y, &level_heads, &current_heads);

    let mut flood_map: Map = vec![vec![0; map_max_size]; map_max_size];
    let steps = flood(&map, (orig_x, orig_y), &HashSet::new(), &mut flood_map);

    //print_flood_map(&map, &flood_map, orig_x, orig_y, min_x, max_x, min_y, max_y);

    println!("max steps: {} largest number of doors: {}", steps, steps / 2);

    let mut num_of_far_rooms = 0;
    for n in min_y..=max_y {
        for m in min_x..=max_x {
            if map[m][n] == 0 && flood_map[m][n] >=2000 {
                num_of_far_rooms += 1;
            }
        }
    }
    println!("number of rooms pass through at least 1000 doors: {}", num_of_far_rooms);
}

fn mark_map(map: &mut Map, x: usize, y:usize, direction: u8) -> (usize, usize) {
    match direction as char {
        'N' => {
            map[x][y - 1] = 2;
            map[x][y - 2] = 0;
            (x, y - 2)
        }
        'S' => {
            map[x][y + 1] = 2;
            map[x][y + 2] = 0;
            (x, y + 2)
        }
        'W' => {
            map[x - 1][y] = 1;
            map[x - 2][y] = 0;
            (x - 2, y)
        }
        'E' => {
            map[x + 1][y] = 1;
            map[x + 2][y] = 0;
            (x + 2, y)
        }
        _ => (0, 0)
    }
}

fn print_map(map: &Map, orig_x: usize, orig_y: usize, min_x: usize, max_x: usize, min_y: usize, max_y: usize, level_heads: &HashSet<(usize, usize)>, current_heads: &HashSet<(usize, usize)>) {
    //let map_x_size = map.len();
    //let map_y_size = map[0].len();

    for n in min_y - 1..=max_y + 1 {
        for m in min_x - 1..=max_x + 1 {
            if m == orig_x && n == orig_y {
                print!("X");
            } else if level_heads.contains(&(m, n)) {
                print!("o");
            } else if current_heads.contains(&(m, n)) {
                print!("*");
            } else {
            match map[m][n] {
            0 => print!("."),
            1 => print!("|"),
            2 => print!("-"),
            3 => print!("#"),
            _ => ()
            }
            }
        }
        println!("");
    }
    println!("");
}

fn print_flood_map(map: &Map, flood_map: &Map, orig_x: usize, orig_y: usize, min_x: usize, max_x: usize, min_y: usize, max_y: usize) {
    for n in min_y - 1..=max_y + 1 {
        for m in min_x - 1..=max_x + 1 {
            if m == orig_x && n == orig_y {
                print!("X");
            } else {
            match map[m][n] {
            0 | 1 | 2 => print!("{}", flood_map[m][n] % 10),
            //1 => print!("|"),
            //2 => print!("-"),
            3 => print!("#"),
            _ => ()
            }
            }
        }
        println!("");
    }
    println!("");
}

fn flood(map: &Map, start: (usize, usize), ends: &HashSet<(usize, usize)>, flood_map: &mut Map) -> u32 {
    let mut boundary = HashSet::new();
    let mut step = 0;

    flood_map[start.0][start.1] = step;

    if ends.contains(&start) {
        return 0;
    }

    boundary.insert(start);

    'outer: loop {
        let mut new_boundary = HashSet::new();
        for p in boundary {
            let x = p.0;
            let y = p.1;
            
            for (temp_x, temp_y) in [(x - 1, y), (x, y - 1), (x, y + 1), (x + 1, y)].iter() {
                let x1 = *temp_x;
                let y1 = *temp_y;

                if (x1, y1) == start {
                    continue;
                }
                if flood_map[x1][y1] != 0 {
                    continue;
                }
                if map[x1][y1] == 3 && ! ends.contains(&(x1, y1)) {
                    continue;
                }
                new_boundary.insert((x1, y1));
            }
        }

        if new_boundary.is_empty() {
            break;
        }

        step += 1;
        for p in &new_boundary {
            flood_map[p.0][p.1] = step;
        }

        for p in &new_boundary {
            if ends.contains(&p) {
                break 'outer;
            }
        }

        boundary = new_boundary;
    }

    step
}


