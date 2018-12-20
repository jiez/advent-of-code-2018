use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("/home/jie/projects/rust/advent/2/input")
        .expect("Error cannot open the file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Error reading input file");

    let lines: Vec<&str> = contents.lines().collect();

    for i in 0..lines.len() {
        for j in (i+1)..lines.len() {
            if differ_by_one_char(lines[i], lines[j]) {
                println!("{}", lines[i]);
                println!("{}", lines[j]);
            }
        }
    }
}

fn differ_by_one_char(line1: &str, line2: &str) -> bool {
    let mut diff_num = 0;
    let s1 = line1.to_string();
    let s2 = line2.to_string();
    for i in 0..s1.len() {
        if s1.chars().nth(i).unwrap() != s2.chars().nth(i).unwrap() {
            diff_num = diff_num + 1;
            if diff_num > 1 {
                return false;
            }
        }
    }
    if diff_num == 1 {
        return true;
    }
    println!("exactly same {}", line1);
    return false;
}
