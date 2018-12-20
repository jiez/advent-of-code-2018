#[macro_use] extern crate scan_fmt;

use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("/home/jie/projects/rust/advent/day7_1/input")
        .expect("Error cannot open the file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Error reading input file");

    // a map from char -> {char, char, char}
    let mut dependences: HashMap<char, HashSet<char>> = HashMap::new();

    for line in contents.lines() {
        let (dd, ss) = scan_fmt!(line, "Step {[A-Z]} must be finished before step {[A-Z]} can begin.", char, char);
        if let (Some(dep), Some(step)) = (dd, ss) {
//            println!("{} {}", dep, step);
            if dependences.contains_key(&step) {
                (*dependences.get_mut(&step).unwrap()).insert(dep);
            } else {
                let mut deps = HashSet::new();
                deps.insert(dep);
                dependences.insert(step, deps);
            }

            if ! dependences.contains_key(&dep) {
                let mut empty = HashSet::new();
                dependences.insert(dep, empty);
            }
        }
    }

    'outer: loop {

    let a = 'A' as u8;
    let z = 'Z' as u8;
    for s in a..=z {
        let step = s as char;
        if dependences.contains_key(&step) {
            if (*dependences.get_mut(&step).unwrap()).is_empty() {
                print!{"{}", step};
                dependences.remove(&step);
                for (_, deps) in dependences.iter_mut() {
                    deps.remove(&step);
                }
                continue 'outer;
            }
        }
    }

    println!("");

    break;
    }
/*
    let A = 'A' as u8;
    let Z = 'Z' as u8;
    for s in A..=Z {
        let step = s as char;
        if dependences.contains_key(&step) {
        print!("{}:", step);
            for dep in dependences.get(&step).unwrap() {
                print!(" {}", dep);
            }
        }
        println!("");
    }
*/
}

