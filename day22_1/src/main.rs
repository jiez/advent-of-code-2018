
fn main() {
    let depth = 3339;
    let target_x = 10;
    let target_y = 715;
    let mut geo_idx = vec![vec![0; target_y + 1]; target_x + 1];
    let mut erosion_lvl = vec![vec![0; target_y + 1]; target_x + 1];
    let mut map = vec![vec![0; target_y + 1]; target_x + 1];

    geo_idx[0][0] = 0;
    geo_idx[target_x][target_y] = 0;
    for i in 1..=target_x {
        geo_idx[i][0] = i * 16807;
    }
    for j in 1..=target_y {
        geo_idx[0][j] = j * 48271;
    }

    erosion_lvl[0][0] = (geo_idx[0][0] + depth) % 20183;
    for i in 1..=target_x {
        erosion_lvl[i][0] = (geo_idx[i][0] + depth) % 20183;
    }
    for j in 1..=target_y {
        erosion_lvl[0][j] = (geo_idx[0][j] + depth) % 20183;
    }
    for i in 1..=target_x {
        for j in 1..=target_y {
            if i != target_x || j != target_y {
                geo_idx[i][j] = erosion_lvl[i - 1][j] * erosion_lvl[i][j - 1];
            }
            erosion_lvl[i][j] = (geo_idx[i][j] + depth) % 20183;
        }
    }

    let mut risk_lvl = 0;
    for i in 0..=target_x {
        for j in 0..=target_y {
            map[i][j] = erosion_lvl[i][j] % 3;
            risk_lvl += map[i][j];
        }
    }
    println!("risk level = {}", risk_lvl);
}
