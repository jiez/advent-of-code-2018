use std::fs::File;
use std::io::Read;

type Map = Vec<Vec<u32>>;

fn main() {
    let mut file = File::open("/home/jie/projects/rust/advent/day18_1/input")
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

    let mut map: Map = vec![vec![0; map_size_y]; map_size_x];

    let mut i = 0;
    for line in contents.lines() {
        for j in 0..line.len() {
            //println!("{}, {}", i, j);
            match &line[j..j+1] {
            "." => map[i][j] = 0,
            "|" => map[i][j] = 1,
            "#" => map[i][j] = 2,
            _ => ()
            }
        }
        i += 1;
    }

    let mut last_min = 0;
//    print_map(&map);
    for min in 1..=10000 {
        let mut temp_map: Map = vec![vec![0; map_size_y]; map_size_x];
        for i in 0..map_size_x {
            for j in 0..map_size_y {
                temp_map[i][j] = next_minute(&map, i, j);
            }
        }

        map = temp_map;
//        print_map(&map);
    }
    let mut saved_map: Map = vec![vec![0; map_size_y]; map_size_x];
    for i in 0..map_size_x {
        for j in 0..map_size_y {
            saved_map[i][j] = map[i][j];
        }
    }
    for min in 10001..=20000 {
        let mut temp_map: Map = vec![vec![0; map_size_y]; map_size_x];
        for i in 0..map_size_x {
            for j in 0..map_size_y {
                temp_map[i][j] = next_minute(&map, i, j);
            }
        }

        map = temp_map;

        if saved_map == map {
            last_min = min;
            break;
        }
    }

    let cycle = last_min - 10000;
    let remainder = (1000000000 - 10000) % cycle;

    for min in 10001..=10000 + remainder {
        let mut temp_map: Map = vec![vec![0; map_size_y]; map_size_x];
        for i in 0..map_size_x {
            for j in 0..map_size_y {
                temp_map[i][j] = next_minute(&map, i, j);
            }
        }

        map = temp_map;
    }


    let mut total_tree_acres = 0;
    let mut total_lumber_acres = 0;
    for i in 0..map_size_x {
        for j in 0..map_size_y {
            if map[i][j] == 1 {
                total_tree_acres += 1;
            } else if map[i][j] == 2 {
                total_lumber_acres += 1;
            }
        }
    }

    println!("{} * {} = {}", total_tree_acres, total_lumber_acres, total_tree_acres * total_lumber_acres);
}

fn count_adjacent_acres(map: &Map, i: usize, j: usize) -> (u32, u32) {
    let map_size_x = map.len();
    let map_size_y = map[0].len();
    let x = i as i32;
    let y = j as i32;
    let mut tree_acres = 0;
    let mut lumber_acres = 0;

    for (xx, yy) in [(x - 1, y - 1), (x - 1, y), (x - 1, y + 1), (x, y - 1), (x, y + 1), (x + 1, y - 1), (x + 1, y), (x + 1, y + 1)].iter() {
        let x1 = *xx;
        let y1 = *yy;
        if x1 >= 0 && y1 >= 0 && x1 < map_size_x as i32 && y1 < map_size_x as i32{
            if map[x1 as usize][y1 as usize] == 1 {
                tree_acres += 1;
            } else if map[x1 as usize][y1 as usize] == 2 {
                lumber_acres += 1;
            }
        }
    }
    (tree_acres, lumber_acres)
} 

fn next_minute(map: &Map, i: usize, j: usize) -> u32 {
    let (tree_acres, lumber_acres) = count_adjacent_acres(map, i, j);
    if map[i][j] == 0 && tree_acres >= 3 {
        1
    } else if map[i][j] == 1 && lumber_acres >= 3 {
        2
    } else if map[i][j] == 2 && !(lumber_acres >= 1 && tree_acres >=1) {
        0
    } else {
        map[i][j]
    }
}

fn print_map(map: &Map) {
    let map_size_x = map.len();
    let map_size_y = map[0].len();

    for m in 0..map_size_x {
        for n in 0..map_size_y {
            match map[m][n] {
            0 => print!("."),
            1 => print!("|"),
            2 => print!("#"),
            _ => ()
            }
        }
        println!("");
    }
    println!("");
}
