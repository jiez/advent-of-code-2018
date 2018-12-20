#[macro_use] extern crate scan_fmt;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("/home/jie/projects/rust/advent/day12_1/input")
        .expect("Error cannot open the file");
    let mut contents = String::new();
    let mut pots = String::new();
    let mut rules = vec![0; 32];

    file.read_to_string(&mut contents)
        .expect("Error reading input file");


    for line in contents.lines() {
        let pots_str = scan_fmt!(line, "initial state: {/[.#]*/}", String);
        if let Some(mut pots_init) = pots_str {
            pots.push_str(pots_init.as_mut_str().replace(".", "0").as_mut_str().replace("#", "1").as_str());
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

    let mut first_pot_index: i64 = 0;
    let mut step: i64 = 0;
    let mut last_i = 1 as i64;

    for i in 1..=50000000000 as i64 {
        let prev_pots = pots.clone();
        let prev_first_pot_index = first_pot_index;
        let mut new_pots = String::new();
        let mut temp_pots = String::from("0000");
        temp_pots.push_str(&pots[..]);
        temp_pots.push_str("0000");
        pots = temp_pots;
        for j in 0..=(pots.len() - 5) {
            let condition_idx = usize::from_str_radix(&pots[j..j+5], 2).unwrap();
            new_pots.push_str(&rules[condition_idx].to_string());
        }
//        println!("{}", new_pots);
        pots = new_pots;
        first_pot_index -= 2;

        let len_before_trim = pots.len();
        pots = pots.trim_start_matches('0').to_string();
        first_pot_index += (len_before_trim - pots.len()) as i64;
        pots = pots.trim_end_matches('0').to_string();
//        println!("{} {}", first_pot_index, pots);
        last_i = i;
        if pots == prev_pots {
            step = first_pot_index - prev_first_pot_index;
            break;
        }
    }

    first_pot_index += (50000000000 - last_i) * step;
    let mut sum: i64 = 0;
    for i in 0..pots.len() {
        if usize::from_str_radix(&pots[i..i+1], 2).unwrap() == 1 {
            let idx: i64 = i as i64;
            sum += idx + first_pot_index;
        }
    }

    println!("sum = {}", sum);
}
