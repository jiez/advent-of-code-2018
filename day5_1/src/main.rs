use std::fs::File;
use std::io::Read;

fn is_opposite_polarity(a: u8, b: u8) -> bool {
    if a > b && a - b == 32 { return true; }
    if a < b && b - a == 32 { return true; }
    return false;
}

fn main() {
    let mut file = File::open("/home/jie/projects/rust/advent/day5_1/input")
        .expect("Error cannot open the file");
    let mut polymers = String::new();

    file.read_to_string(&mut polymers)
        .expect("Error reading input file");

//    println!("{}", polymers.lines().next().unwrap().len());

    let mut polymers_bytes = polymers.lines().next().unwrap().to_string().into_bytes();

    let mut i = 0 as usize;

    loop {
        if i + 1 >= polymers_bytes.len() {
            break;
        }
//        println!("{}", i);
//        println!("{} {}", polymers_bytes[i], polymers_bytes[i + 1]);
        if is_opposite_polarity(polymers_bytes[i], polymers_bytes[i + 1]) {
            polymers_bytes.remove(i + 1);
            polymers_bytes.remove(i);
            if i > 0 {
                i = i - 1;
            }
        } else {
            i = i + 1;
        }
    }

   //println!("{}", String::from_utf8(polymers_bytes).unwrap().len());
   println!("{}", polymers_bytes.len());
}
