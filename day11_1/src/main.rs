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
    for i in 1..=298 {
        for j in 1..=298 {
            let mut total: i32 = 0;

            total += cells[i][j] + cells[i][j+1] + cells[i][j+2]
                + cells[i+1][j] + cells[i+1][j+1] + cells[i+1][j+2]
                + cells[i+2][j] + cells[i+2][j+1] + cells[i+2][j+2];
            if total > max {
                max = total;
                max_x = i;
                max_y = j;
            }
        }
    }

    println!("({}, {}), {}", max_x, max_y, max);
    
}
