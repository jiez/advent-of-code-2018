type Map = Vec<Vec<usize>>;
type ExploreMap = Vec<Vec<Vec<(usize, usize, u32)>>>;

fn main() {
/*
    let depth = 510;
    let target_x = 10;
    let target_y = 10;
*/
    let depth = 3339;
    let target_x = 10;
    let target_y = 715;

    let map_size_x = target_x + 10;
    let map_size_y = target_y + 10;
    let mut geo_idx = vec![vec![0; map_size_y]; map_size_x];
    let mut erosion_lvl = vec![vec![0; map_size_y]; map_size_x];
    let mut map: Map = vec![vec![0; map_size_y]; map_size_x];
    let mut explore_map = vec![vec![vec![(0, 0, std::u32::MAX); 3]; map_size_y]; map_size_x];
    explore_map[target_x][target_y][0] = (target_x, target_y, 0);
    explore_map[target_x][target_y][1] = (target_x, target_y, 7);
    explore_map[target_x][target_y][2] = (target_x, target_y, 7);

    geo_idx[0][0] = 0;
    geo_idx[target_x][target_y] = 0;
    for i in 1..map_size_x {
        geo_idx[i][0] = i * 16807;
    }
    for j in 1..map_size_y {
        geo_idx[0][j] = j * 48271;
    }

    erosion_lvl[0][0] = (geo_idx[0][0] + depth) % 20183;
    for i in 1..map_size_x {
        erosion_lvl[i][0] = (geo_idx[i][0] + depth) % 20183;
    }
    for j in 1..map_size_y {
        erosion_lvl[0][j] = (geo_idx[0][j] + depth) % 20183;
    }
    for i in 1..map_size_x {
        for j in 1..map_size_y {
            if i != target_x || j != target_y {
                geo_idx[i][j] = erosion_lvl[i - 1][j] * erosion_lvl[i][j - 1];
            }
            erosion_lvl[i][j] = (geo_idx[i][j] + depth) % 20183;
        }
    }

    //let mut risk_lvl = 0;
    for i in 0..map_size_x {
        for j in 0..map_size_y {
            map[i][j] = erosion_lvl[i][j] % 3;
            //risk_lvl += map[i][j];
        }
    }
    //println!("risk level = {}", risk_lvl);
    print_map(&map, target_x, target_y);

    //print_explore_map(&explore_map);
    let mut count = 0;
    loop {
        if explore_map[0][0][0].2 != std::u32::MAX {
            println!("count = {}, shortest time: {}", count, explore_map[0][0][0].2);
        }
        if ! explore(&map, &mut explore_map) {
            break;
        }
        count += 1;
        //print_explore_map(&explore_map);
    }

    println!("shortest time: {}", explore_map[0][0][0].2);
}

fn print_map(map: &Map, target_x: usize, target_y: usize) {
    let map_x_size = map.len();
    let map_y_size = map[0].len();

    for n in 0..map_y_size {
        for m in 0..map_x_size {
            if m == target_x && n == target_y {
                print!("T");
            } else {
            match map[m][n] {
            0 => print!("."),
            1 => print!("="),
            2 => print!("|"),
            _ => ()
            }
            }
        }
        println!("");
    }
    println!("");
}

fn print_explore_map(map: &ExploreMap) {
    let map_x_size = map.len();
    let map_y_size = map[0].len();

    for n in 0..map_y_size {
        for m in 0..map_x_size {
            print!("[");
            for k in 0..3 {
                if map[m][n][k].2 == std::u32::MAX {
                    print!("(__,__)__");
                } else {
                    print!("({:2},{:2}){:2}", map[m][n][k].0, map[m][n][k].1, map[m][n][k].2);
                }
                if k != 2 { print!(" ")}
            }
            print!("]");
        }
        println!("");
    }
    println!("");
}

fn explore(map: &Map, explore_map: &mut ExploreMap) -> bool {
    let map_size_x = map.len();
    let map_size_y = map[0].len();
    let mut changed = false;

    for j in (0..map_size_y).rev() {
        for i in 0..map_size_x {
            for t in 0..3 {

                if ! usable(map[i][j], t) {
                    continue;
                }

                let x = i as i32;
                let y = j as i32;
                let mut to_x = 0;
                let mut to_y = 0;
                let mut min_minutes = std::u32::MAX;
                for (a, b) in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)].iter() {
                    if *a < 0 || *b < 0 {
                        continue;
                    }
                    let x1 = *a as usize;
                    let y1 = *b as usize;
                    if x1 >= map_size_x || y1 >= map_size_y {
                        continue;
                    }
                    for t1 in 0..3 {
                        let mut minutes = explore_map[x1][y1][t1].2;
                        if minutes == std::u32::MAX {
                            continue;
                        }
                        minutes += 1;
                        if t != t1 {
                            minutes += 7;
                        }
                        if minutes < min_minutes {
                            min_minutes = minutes;
                            to_x = x1;
                            to_y = y1;
                        }
                    }
                }
                if min_minutes < explore_map[i][j][t].2 {
                    explore_map[i][j][t].0 = to_x;
                    explore_map[i][j][t].1 = to_y;
                    explore_map[i][j][t].2 = min_minutes;
                    changed = true;
                }
            }
        }
    }

    changed
}

fn usable (region_type: usize, tool: usize) -> bool {
    let mut result = true;
    if region_type == 0 && tool == 2 {
        result = false;
    } else if region_type == 1 && tool == 0 {
        result = false;
    } else if region_type == 2 && tool == 1 {
        result = false;
    }
    result
}
