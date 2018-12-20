use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut count2_num = 0;
    let mut count3_num = 0;
    let mut count1_letters = HashSet::new();
    let mut count2_letters = HashSet::new();
    let mut count3_letters = HashSet::new();
    let mut count4_or_more_letters = HashSet::new();
    let mut file = File::open("/home/jie/projects/rust/advent/2/input")
        .expect("Error cannot open the file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Error reading input file");

    for line in contents.lines() {
        for c in line.chars() {
            if count4_or_more_letters.contains(&c) {
                continue;
            } else if count3_letters.contains(&c) {
                count3_letters.remove(&c);
                count4_or_more_letters.insert(c);
            } else if count2_letters.contains(&c) {
                count2_letters.remove(&c);
                count3_letters.insert(c);
            } else if count1_letters.contains(&c) {
                count1_letters.remove(&c);
                count2_letters.insert(c);
            } else {
                count1_letters.insert(c);
            }
        }

        println!("{}: {} {} {} {}", line, count4_or_more_letters.len(), count3_letters.len(), count2_letters.len(), count1_letters.len());

        if ! count2_letters.is_empty() {
            count2_num = count2_num + 1;
        }
        if ! count3_letters.is_empty() {
            count3_num = count3_num + 1;
        }

        count4_or_more_letters.clear();
        count3_letters.clear();
        count2_letters.clear();
        count1_letters.clear();
    }
    println!("count2 x count3 = {}", count2_num * count3_num);
}
