#[macro_use] extern crate scan_fmt;

use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::cmp::Ordering;
use std::process;

struct Group {
    army: String,
    num: i32,
    hit_points: i32,
    immunities: HashSet<String>,
    weaks: HashSet<String>,
    attack_amount: i32,
    attack_type: String,
    initiative: i32,
    attack_target: usize,
}

impl Ord for Group {
    fn cmp(&self, other: &Group) -> Ordering {
        if self.num * self.attack_amount < other.num * other.attack_amount {
            Ordering::Less
        } else if self.num * self.attack_amount > other.num * other.attack_amount {
            Ordering::Greater
        } else if self.initiative < other.initiative {
            Ordering::Less
        } else if self.initiative > other.initiative {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Group) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Group {
    fn eq(&self, other: &Group) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Group {}


fn main() {
    let mut file = File::open("/home/jie/projects/rust/advent/day24_1/input")
        .expect("Error cannot open the file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Error reading input file");

    let mut boost = 200;
    'outest: loop {
    let mut groups = Vec::new();
    println!("boost = {}", boost);

    let mut current_army = String::from("");
    for line in contents.lines() {
        if line == "Immune System:" {
            current_army = String::from("Immune System");
            continue;
        }

        if line == "Infection:" {
            current_army = String::from("Infection");
            continue;
        }

        if line == "" {
            continue;
        }

        let pieces: Vec<&str> = line.split(|c| c == '(' || c == ')').collect();
        let mut input = pieces[0].to_owned();
        if pieces.len() == 3 {
            input.push_str(pieces[2]);
        }

        //let mut group = Group{army: String::new(), num: 0, hit_points: 0, immunities: HashSet::new(), weaks: HashSet::new(), attack_amount: 0, attack_type: String::new(), initiative: 0, attack_target: std::usize::MAX};
        let mut group;
        let (a, b, c, d, e)  = scan_fmt!(&input, "{d} units each with {d} hit points with an attack that does {d} {} damage at initiative {d}", i32, i32, i32, String, i32);
        if let (Some(num), Some(hit_points), Some(attack_amount), Some(attack_type), Some(initiative)) = (a, b, c, d, e) {
            //println!("{} {} {} {} {}", num, hit_points, attack_amount, attack_type, initiative);
            group = Group{army: current_army.clone(), num: num, hit_points: hit_points, immunities: HashSet::new(), weaks: HashSet::new(), attack_amount: attack_amount, attack_type: attack_type, initiative: initiative, attack_target: std::usize::MAX};
        } else {
            println!("cannot parse input line {}", line);
            process::exit(1);
        }

        if pieces.len() == 3 {
            let immune_and_weaks: Vec<&str> = pieces[1].split(';').collect();
            for i in 0..immune_and_weaks.len() {
                let mut immune_or_weaks;
                if immune_and_weaks[i].trim_start().starts_with("immune to") {
                    immune_or_weaks = &mut group.immunities;
                } else {
                    immune_or_weaks = &mut group.weaks;
                }
                let attacks: Vec<&str> = immune_and_weaks[i].split(|c| c == ',' || c == ' ').collect();
                for attack in attacks {
                    match attack {
                        "radiation" | "cold" | "fire" | "slashing" | "bludgeoning" => { immune_or_weaks.insert(attack.to_string()); }
                        _ => (),
                    }
                }
            }
        }

        groups.push(group);
    }

    //print_groups(&groups);

        for i in 0..groups.len() {
            if groups[i].army == "Immune System" {
                groups[i].attack_amount += boost;
            }
        }

    'outer: loop {

    groups.sort_by(|a, b| b.cmp(a));

    //print_groups(&groups);

    // select the target
    let mut targets = vec![std::usize::MAX; groups.len()];
    for i in 0..groups.len() {
        //println!("{} selecting target...", i);
        let mut target = std::usize::MAX;
        let mut max_damage = 0;
        for j in 0..groups.len() {
            if i == j {
                continue;
            }
            if groups[i].army == groups[j].army {
                continue;
            }
            if targets.contains(&j) {
                //println!("  {} has been selected by other", j);
                continue;
            }
            if groups[j].immunities.contains(&groups[i].attack_type) {
                //println!("  {} is immune", j);
                continue;
            }
            let mut effective_power = groups[i].num * groups[i].attack_amount;
            if groups[j].weaks.contains(&groups[i].attack_type) {
                //println!("doubled!");
                effective_power *= 2;
            }
            if target == std::usize::MAX {
                //println!("  {} is first to select", j);
                max_damage = effective_power;
                target = j;
                continue;
            }
            if max_damage > effective_power {
                //println!("  {} will receive less damage", j);
                continue;
            }
            if max_damage < effective_power {
                //println!("  {} will receive more damage, select", j);
                max_damage = effective_power;
                target = j;
                continue;
            }
            // max_damage == effective_power
            if groups[j].num * groups[j].attack_amount < groups[target].num * groups[target].attack_amount {
                //println!("  {} has less effective power than the selected one", j);
                continue;
            }
            if groups[j].num * groups[j].attack_amount > groups[target].num * groups[target].attack_amount {
                //println!("  {} has more effective power than the previous selected one, select", j);
                target = j;
                continue;
            }
            // even effective powers tie
            if groups[j].initiative > groups[target].initiative {
                //println!("  {} has same effective power as the previous selected one, but has higher initiative, select", j);
                target = j;
            }
        }
        targets[i] = target;
    }

/*
    println!("choosing target:");
    for i in 0..targets.len() {
        print!("{}: ", i);
        if targets[i] == std::usize::MAX {
            println!("");
        } else {
            println!("{}", targets[i]);
        }
    }
*/

    for i in 0..groups.len() {
        groups[i].attack_target = targets[i];
    }

    // attack

    let mut attack_order = Vec::new();
    for i in 0..groups.len() {
        attack_order.push((i, groups[i].initiative));
    }
    attack_order.sort_by(|a, b| b.1.cmp(&a.1));
    
    for k in 0..attack_order.len() {
        let i = attack_order[k].0;
        let target = groups[i].attack_target;
        let mut effective_power = groups[i].num * groups[i].attack_amount;
        if target == std::usize::MAX {
            continue;
        }
        //println!("{} attacks {}", i, target);
        if groups[target].weaks.contains(&groups[i].attack_type) {
            //println!("doubled!");
            effective_power *= 2;
        }
        //println!("  could kill {} units", effective_power / groups[target].hit_points);
        groups[target].num -= effective_power / groups[target].hit_points;
        if groups[target].num < 0 {
            groups[target].num = 0;
        }
    }

    groups.retain(|g| g.num > 0);

    for i in 1..groups.len() {
        if groups[i].army != groups[0].army {
            continue 'outer;
        }
    }

    break;
    }

    if groups[0].army == "Infection" {
        boost -= 1;
        continue;
    } else {
        boost -= 1;
    }


    let mut num_of_units = 0;
    for i in 0..groups.len() {
        num_of_units += groups[i].num;
    }

    println!("immune system will have {} units remaining with boost {}", num_of_units, boost);
    }
}

fn print_groups(groups: &Vec<Group>) {
    for group in groups {
        print!("{} {} {} {} {} {} effective power {}", group.army, group.num, group.hit_points, group.attack_amount, group.attack_type, group.initiative, group.num * group.attack_amount);
        print!(" weaks:");
        for weak in &group.weaks {
            print!(" {}", weak);
        }
        print!(" immunities:");
        for immunity in &group.immunities {
            print!(" {}", immunity);
        }
        println!("");
    }
    println!("");
}
