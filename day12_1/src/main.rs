#[macro_use] extern crate scan_fmt;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("/home/jie/projects/rust/advent/day12_1/input")
        .expect("Error cannot open the file");
    let mut contents = String::new();
    let mut pots = String::from("000000000000000000000000");
    let mut rules = vec![0; 32];

    file.read_to_string(&mut contents)
        .expect("Error reading input file");


    for line in contents.lines() {
        let pots_str = scan_fmt!(line, "initial state: {/[.#]*/}", String);
        if let Some(mut pots_init) = pots_str {
            pots.push_str(pots_init.as_mut_str().replace(".", "0").as_mut_str().replace("#", "1").as_str());
            pots.push_str("000000000000000000000000");
        } else {
            let (condition_str, result_str) = scan_fmt!(line, "{/[.#]*/} => {[.#]}", String, String);
            if let (Some(mut condition), Some(result)) = (condition_str, result_str) {
                if result == "#" {
                    let condition_bin = condition.as_mut_str().replace(".", "0").as_mut_str().replace("#", "1");
                    let condition_idx = usize::from_str_radix(condition_bin.as_str(), 2).unwrap();
                    rules[condition_idx] = 1;
                }
            }
        }
    }

    println!("{}", pots);

    for _i in 1..=20 {
        let mut new_pots = String::from("00");
        for j in 0..=(pots.len() - 5) {
            let condition_idx = usize::from_str_radix(&pots[j..j+5], 2).unwrap();
            new_pots.push_str(&rules[condition_idx].to_string());
        }
        new_pots.push_str("00");
        println!("{}", new_pots);
        pots = new_pots;
    }

    let mut sum = 0;
    for i in 0..pots.len() {
        if usize::from_str_radix(&pots[i..i+1], 2).unwrap() == 1 {
            sum += i - 24;
        }
    }

    println!("sum = {}", sum);
}
