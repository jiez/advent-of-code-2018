fn power_level(x: usize, y: usize, sn: i32) -> i32 {
    let mut level: i32;
    let xx = x as i32;
    let yy = y as i32;

    level = ((xx + 10) * yy + sn) * (xx + 10);
    level /= 100;
    level -= (level / 10) * 10;
    level -= 5;
    level
}

fn main() {
    let mut cells: Vec<Vec<i32>> = vec![vec![0; 301]; 301];
    let sn: i32 = 4172;

/*
    println!("{}", power_level(3, 5, 8));
    println!("{}", power_level(122, 79, 57));
    println!("{}", power_level(217, 196, 39));
    println!("{}", power_level(101, 153, 71));
*/

    for i in 1..=300 {
        for j in 1..=300 {
            cells[i][j] = power_level(i, j, sn);
        }
    }

    let mut max: i32 = std::i32::MIN;
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    let mut max_s: usize = 0;
    for s in 1..=300 {
    println!("size {}", s);
    for i in 1..=(300 - s + 1) {
        for j in 1..=(300 - s + 1) {
            let mut total: i32 = 0;

            for k in 0..s {
                for l in 0..s {
                    total += cells[i+k][j+l];
                }
            }

            if total > max {
                max = total;
                max_x = i;
                max_y = j;
                max_s = s;
            }
        }
    }
    }

    println!("({}, {}, {}), {}", max_x, max_y, max_s, max);
    
}
