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

    let worker_num = 5;
    let additional_time = 60;
    let mut second = 0;
    let mut workers = vec![0; worker_num];
    let mut workers_work: Vec<char> = vec!['_'; worker_num];

    'outer: loop {

    'next_worker: for w in 0..worker_num {
        if workers[w] == 0 {
//println!("looking for a job for worker {}", w);
            let a = 'A' as u8;
            let z = 'Z' as u8;
            'next_work: for s in a..=z {
                let step = s as char;
                if dependences.contains_key(&step) {
                    if (*dependences.get_mut(&step).unwrap()).is_empty() {
                        // if some worker is already working on it
                        // try to find the next one
//println!("  {} is available", step);
                        for w1 in 0..worker_num {
                            if workers_work[w1] == step {
//println!("    but already taken by {}", w1);
                                continue 'next_work;
                            }
                        }
                        //print!{"{}", step};
                        dependences.remove(&step);
                        workers[w] = s - a + 1 + additional_time;
                        workers_work[w] = step;
                        /*
                        */
                        continue 'next_worker;
                    }
                }
            }
        }
    }

//    println!("{}  {} {}", second, workers_work[0], workers_work[1]);
    println!("{}  {} {} {} {} {}", second, workers_work[0], workers_work[1], workers_work[2], workers_work[3], workers_work[4]);
//    println!("{}  {} {} {} {} {}", second, workers[0], workers[1], workers[2], workers[3], workers[4]);


    let mut sum: u32 = 0;
    for w in 0..worker_num {
        let ww = workers[w] as u32;
        sum += ww;
    }

    // if no more work, done!
    if sum == 0 {
        break 'outer;
    }

//println!("second ++");
    second += 1;
    for w in 0..worker_num {
        if workers[w] > 0 {
            workers[w] -= 1;
            if workers[w] == 0 {
//                println!("{} is done", workers_work[w]);
                for (_, deps) in dependences.iter_mut() {
                    deps.remove(&workers_work[w]);
                }
                workers_work[w] = '_';
            }
        }
    }
                

    }

    println!("{}", second);
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

