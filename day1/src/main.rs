use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut freq = 0;
    let mut freqs = HashSet::new();
    let mut found = false;
    let mut file = File::open("/home/jie/projects/rust/advent/1/input")
        .expect("Error cannot open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading input file");
    freqs.insert(freq);
    while !found {
        for line in contents.lines() {
            freq += line.parse().unwrap();
            if freqs.contains(&freq) {
                found = true;
                break;
            }
            freqs.insert(freq);
        }
    }
    println!("first twice freq = {}", freq);
}
