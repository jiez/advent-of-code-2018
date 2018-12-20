use std::fs::File;
use std::io::Read;
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(PartialEq)]
enum UnitType {
    Goblin,
    Elf,
}

struct Unit {
    x: usize,
    y: usize,
    unit_type: UnitType,
    hit_points: i32,
}

impl Ord for Unit {
    fn cmp(&self, other: &Unit) -> Ordering {
        if self.x < other.x {
            Ordering::Less
        } else if self.x > other.x {
            Ordering::Greater
        } else if self.y < other.y {
            Ordering::Less
        } else if self.y > other.y {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Unit) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Unit {
    fn eq(&self, other: &Unit) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Unit {}

type Map = Vec<Vec<u32>>;

fn main() {
    let init_hit_points = 200;
    let goblin_attack_power = 3;
    let mut elf_attack_power = 3;

    let mut file = File::open("/home/jie/projects/rust/advent/day15_1/input")
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

    'outest: loop {

    let mut map: Map = vec![vec![0; map_size_y]; map_size_x];

    let mut units = Vec::new();

    let mut init_num_of_goblins = 0;
    let mut init_num_of_elves = 0;

    let mut i = 0;
    for line in contents.lines() {
        for j in 0..line.len() {
            //println!("{}, {}", i, j);
            match &line[j..j+1] {
            "." => map[i][j] = 0,
            "#" => map[i][j] = 1,

            // we use 0x10 to mark the cart being at this place
            "G" => {
                map[i][j] = 2;
                let unit: Unit = Unit{ x: i, y: j, unit_type: UnitType::Goblin, hit_points: init_hit_points};
                units.push(unit);
                init_num_of_goblins += 1;
            }
            "E" => {
                map[i][j] = 3;
                let unit: Unit = Unit{ x: i, y: j, unit_type: UnitType::Elf, hit_points: init_hit_points};
                units.push(unit);
                init_num_of_elves += 1;
            }
            _ => ()
            }
        }
        i += 1;
    }

    let mut num_of_goblins = init_num_of_goblins;
    let mut num_of_elves = init_num_of_elves;
    let mut round = 0;

    'outer: loop {

    let num_of_units = units.len();
    for i in 0..num_of_units {
        println!("round = {} unit = {}: start ...", round, i);

        // if this unit is already dead
        if units[i].hit_points <= 0 {
            println!("round = {} unit = {}: dead unit", round, i);
            continue;
        }

        // if this unit is not adjacent to an enemy, it will need to move toward the nearest one of them
        let target_num = if units[i].unit_type == UnitType::Goblin { 3 } else { 2 };

        // if no enemies left, done
        if target_num == 2 && num_of_goblins == 0 {
            break 'outer;
        } else if target_num == 3 && num_of_elves == 0 {
            break 'outer;
        }

        if map[units[i].x - 1][units[i].y] != target_num
           && map[units[i].x][units[i].y - 1] != target_num
           && map[units[i].x][units[i].y + 1] != target_num
           && map[units[i].x + 1][units[i].y] != target_num {

        println!("round = {} unit = {}: do not have adjacent target, need to move", round, i);

        // find all attack positions
        let target_type = if units[i].unit_type == UnitType::Goblin { UnitType::Elf } else { UnitType::Goblin };
        let mut attack_pos_set = HashSet::new();
        for j in 0..num_of_units {
            if units[j].unit_type == target_type && units[j].hit_points > 0 {
                let x = units[j].x;
                let y = units[j].y;
                for (temp_x, temp_y) in [(x - 1, y), (x, y - 1), (x, y + 1), (x + 1, y)].iter() {
                    // we don't need to worry if these coordinaes will be out of map because of walls on the perimeter
                    if map[*temp_x][*temp_y] == 0 {
                        attack_pos_set.insert((*temp_x, *temp_y));
                    }
                }
            }
        }

        if attack_pos_set.is_empty() {
            println!("round = {} unit = {}: could not find possible attack positions", round, i);
            continue;
        }

        println!("round = {} unit = {}: possible attack positions", round, i);
        for m in 0..map_size_x {
            for n in 0..map_size_y {
                if attack_pos_set.contains(&(m, n)) {
                    print!("?");
                    continue;
                }
                match map[m][n] {
                0 => print!("."),
                1 => print!("#"),
                2 => print!("G"),
                3 => print!("E"),
                _ => ()
                }
            }
            println!("");
        }
        println!("");

        // find the nearest attack position, i.e. target position this unit will move to
        let mut target_flood_map: Map = vec![vec![0; map_size_y]; map_size_x];
        flood(&map, (units[i].x, units[i].y), &attack_pos_set, &mut target_flood_map);

        println!("round = {} unit = {}: flood to find the nearest attack position", round, i);

        for m in 0..map_size_x {
            for n in 0..map_size_y {
                if (m, n) == (units[i].x, units[i].y) {
                    print!("*");
                    continue;
                } else if target_flood_map[m][n] != 0 {
                    print!("{}",target_flood_map[m][n]%10);
                    continue;
                }
                match map[m][n] {
                0 => print!("."),
                1 => print!("#"),
                2 => print!("G"),
                3 => print!("E"),
                _ => ()
                }
            }
            println!("");
        }
        println!("");

        let mut attack_positions = Vec::new();
        for pos in attack_pos_set {
            attack_positions.push(pos);
        }
        attack_positions.sort();

        let mut target_pos = (map_size_x, map_size_y);
        let mut least_steps = std::u32::MAX;
        for pos in attack_positions {
            if target_flood_map[pos.0][pos.1] > 0 && target_flood_map[pos.0][pos.1] < least_steps {
                target_pos = pos;
                least_steps = target_flood_map[pos.0][pos.1];
            }
        }

        if target_pos == (map_size_x, map_size_y) {
            println!("round = {} unit = {}: could not find nearest attack position", round, i);
            continue;
        }

        println!("round = {} unit = {}: found the nearest attack position", round, i);
        for m in 0..map_size_x {
            for n in 0..map_size_y {
                if (m, n) == target_pos {
                    print!("+");
                    continue;
                }
                match map[m][n] {
                0 => print!("."),
                1 => print!("#"),
                2 => print!("G"),
                3 => print!("E"),
                _ => ()
                }
            }
            println!("");
        }
        println!("");

        // move to the target position
        let mut move_pos_set = HashSet::new();
        let x = units[i].x;
        let y = units[i].y;
        for (temp_x, temp_y) in [(x - 1, y), (x, y - 1), (x, y + 1), (x + 1, y)].iter() {
            // we don't need to worry if these coordinaes will be out of map because of walls on the perimeter
            if map[*temp_x][*temp_y] == 0 {
                move_pos_set.insert((*temp_x, *temp_y));
            }
        }
 
        let mut move_flood_map: Map = vec![vec![0; map_size_y]; map_size_x];
        flood(&map, target_pos, &move_pos_set, &mut move_flood_map);

        println!("round = {} unit = {}: flood to move", round, i);
        for m in 0..map_size_x {
            for n in 0..map_size_y {
                if (m, n) == target_pos {
                    print!("*");
                    continue;
                } else if move_flood_map[m][n] != 0 {
                    print!("{}",move_flood_map[m][n] % 10);
                    continue;
                }
                match map[m][n] {
                0 => print!("."),
                1 => print!("#"),
                2 => print!("G"),
                3 => print!("E"),
                _ => ()
                }
            }
            println!("");
        }
        println!("");

        let mut move_positions = Vec::new();
        for pos in move_pos_set {
            move_positions.push(pos);
        }
        move_positions.sort();

        let mut move_pos = (map_size_x, map_size_y);
        let mut move_least_steps = std::u32::MAX;
        for pos in move_positions {
            if pos == target_pos {
                move_pos = pos;
                break;
            }
            if move_flood_map[pos.0][pos.1] > 0 && move_flood_map[pos.0][pos.1] < move_least_steps {
                move_pos = pos;
                move_least_steps = move_flood_map[pos.0][pos.1];
                println!("{} {}: {}", pos.0, pos.1, move_least_steps);
            }
        }

        if move_pos == (map_size_x, map_size_y) {
            println!("round = {} unit = {}: could not find move position", round, i);
            continue;
        }

        println!("round = {} unit = {}: found the move position", round, i);
        for m in 0..map_size_x {
            for n in 0..map_size_y {
                if (m, n) == move_pos {
                    print!("@");
                    continue;
                }
                match map[m][n] {
                0 => print!("."),
                1 => print!("#"),
                2 => print!("G"),
                3 => print!("E"),
                _ => ()
                }
            }
            println!("");
        }
        println!("");

        let temp = map[units[i].x][units[i].y];
        map[units[i].x][units[i].y] = 0;
        units[i].x = move_pos.0;
        units[i].y = move_pos.1;
        map[units[i].x][units[i].y] = temp;

        println!("round = {} unit = {}: after the move", round, i);
        for m in 0..map_size_x {
            for n in 0..map_size_y {
                match map[m][n] {
                0 => print!("."),
                1 => print!("#"),
                2 => print!("G"),
                3 => print!("E"),
                _ => ()
                }
            }
            println!("");
        }
        println!("");

        } // complete moving

        // look for the attack target unit
        let target_num = if units[i].unit_type == UnitType::Goblin { 3 } else { 2 };
        let attack_power = if units[i].unit_type == UnitType::Goblin { goblin_attack_power } else { elf_attack_power };
        let mut least_hit_points = std::i32::MAX;
        let mut target_unit_index = 0;
        let x = units[i].x;
        let y = units[i].y;
        for (temp_x, temp_y) in [(x - 1, y), (x, y - 1), (x, y + 1), (x + 1, y)].iter() {
            let x1 = *temp_x;
            let y1 = *temp_y;
            if map[x1][y1] == target_num {
                let k = find_unit(&units, x1, y1);
                if k == units.len() {
                    continue;
                }
                if least_hit_points > units[k].hit_points {
                    least_hit_points = units[k].hit_points;
                    target_unit_index = k;
                }
            }
        }

        // if we find the attack target unit, attack
        if least_hit_points < std::i32::MAX {
            units[target_unit_index].hit_points -= attack_power;
            println!("round = {} unit = {}: attack unit {} -> new points {}", round, i, target_unit_index, units[target_unit_index].hit_points);
            if units[target_unit_index].hit_points <= 0 {
                // if dead, remove from map
                map[units[target_unit_index].x][units[target_unit_index].y] = 0;
                if target_num == 2 {
                    num_of_goblins -= 1;
                } else {
                    num_of_elves -= 1;
                }
            }
            println!("round = {} unit = {}: after attack", round, i);
            for m in 0..map_size_x {
                for n in 0..map_size_y {
                    match map[m][n] {
                    0 => print!("."),
                    1 => print!("#"),
                    2 => print!("G"),
                    3 => print!("E"),
                    _ => ()
                    }
                }
                println!("");
            }
            println!("");
        } else {
            println!("round = {} unit = {}: no enemy adjacent to attack", round, i);
        }
    }

    round += 1;

    // remove dead units from units vector
    units.retain(|ref u| u.hit_points > 0);

    units.sort();

    for u in &units {
        print!("({},{}):{} ", u.x, u.y, u.hit_points);
    }
    println!("");

    let mut goblin_found = false;
    let mut elf_found = false;
    for u in &units {
        if u.unit_type == UnitType::Goblin {
            goblin_found = true;
        } else {
            elf_found = true;
        }
    }
    if ! goblin_found || ! elf_found {
        break;
    }

    } // 'outer loop

    let mut sum = 0;
    for u in &units {
        if u.hit_points > 0 {
            sum += u.hit_points;
        }
    }

    println!("round = {} sum = {}", round, sum);
    println!("Answer: {} * {} = {}", round, sum, round * sum);

    if num_of_elves == init_num_of_elves {
        println!("no loss of elves");
        break;
    } else {
        println!("loss of elves: {}", init_num_of_elves - num_of_elves);
        elf_attack_power += 1;
    }

    } // 'outest loop

}

fn flood(map: &Map, start: (usize, usize), ends: &HashSet<(usize, usize)>, flood_map: &mut Map) {
    let mut boundary = HashSet::new();
    let mut step = 0;

    flood_map[start.0][start.1] = step;

    if ends.contains(&start) {
        return;
    }

    step += 1;
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
                if map[x1][y1] != 0 && ! ends.contains(&(x1, y1)) {
                    continue;
                }
                new_boundary.insert((x1, y1));
            }
        }

        if new_boundary.is_empty() {
            break;
        }

        for p in &new_boundary {
            flood_map[p.0][p.1] = step;
        }
        step += 1;

        for p in &new_boundary {
            if ends.contains(&p) {
                break 'outer;
            }
        }

        boundary = new_boundary;
    }
}

fn find_unit(units: &Vec<Unit>, x: usize, y: usize) -> usize {
    let num_of_units = units.len();
    let mut found_index = num_of_units;
    for i in 0..num_of_units {
        if units[i].x == x && units[i].y == y && units[i].hit_points > 0 {
            found_index = i;
            break;
        }
     }

    found_index
}

