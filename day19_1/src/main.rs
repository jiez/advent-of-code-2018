#[macro_use] extern crate scan_fmt;

use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::collections::HashMap;

/*
struct Instruction {
    name: String,
    func: fn(regs: &mut Vec<u32>, op0: u32, op1: u32, op2: u32),
}
*/

/*
struct Sample {
    before: (u32, u32, u32, u32),
    after: (u32, u32, u32, u32),
    insn: (u32, u32, u32, u32),
}
*/

fn addr(regs: &mut Vec<usize>, op0: usize, op1: usize, op2: usize) {
    regs[op2 as usize] = regs[op0 as usize] + regs[op1 as usize];
}

fn addi(regs: &mut Vec<usize>, op0: usize, op1: usize, op2: usize) {
    regs[op2 as usize] = regs[op0 as usize] + op1;
}

fn mulr(regs: &mut Vec<usize>, op0: usize, op1: usize, op2: usize) {
    regs[op2 as usize] = regs[op0 as usize] * regs[op1 as usize];
}

fn muli(regs: &mut Vec<usize>, op0: usize, op1: usize, op2: usize) {
    regs[op2 as usize] = regs[op0 as usize] * op1;
}

fn banr(regs: &mut Vec<usize>, op0: usize, op1: usize, op2: usize) {
    regs[op2 as usize] = regs[op0 as usize] & regs[op1 as usize];
}

fn bani(regs: &mut Vec<usize>, op0: usize, op1: usize, op2: usize) {
    regs[op2 as usize] = regs[op0 as usize] & op1;
}

fn borr(regs: &mut Vec<usize>, op0: usize, op1: usize, op2: usize) {
    regs[op2 as usize] = regs[op0 as usize] | regs[op1 as usize];
}

fn bori(regs: &mut Vec<usize>, op0: usize, op1: usize, op2: usize) {
    regs[op2 as usize] = regs[op0 as usize] | op1;
}

fn setr(regs: &mut Vec<usize>, op0: usize, op1: usize, op2: usize) {
    regs[op2 as usize] = regs[op0 as usize];
}

fn seti(regs: &mut Vec<usize>, op0: usize, op1: usize, op2: usize) {
    regs[op2 as usize] = op0;
}

fn gtir(regs: &mut Vec<usize>, op0: usize, op1: usize, op2: usize) {
    regs[op2 as usize] = if op0 > regs[op1 as usize] { 1 } else { 0 };
}

fn gtri(regs: &mut Vec<usize>, op0: usize, op1: usize, op2: usize) {
    regs[op2 as usize] = if regs[op0 as usize] > op1 { 1 } else { 0 };
}

fn gtrr(regs: &mut Vec<usize>, op0: usize, op1: usize, op2: usize) {
    regs[op2 as usize] = if regs[op0 as usize] > regs[op1 as usize] { 1 } else { 0 };
}

fn eqir(regs: &mut Vec<usize>, op0: usize, op1: usize, op2: usize) {
    regs[op2 as usize] = if op0 == regs[op1 as usize] { 1 } else { 0 };
}

fn eqri(regs: &mut Vec<usize>, op0: usize, op1: usize, op2: usize) {
    regs[op2 as usize] = if regs[op0 as usize] == op1 { 1 } else { 0 };
}

fn eqrr(regs: &mut Vec<usize>, op0: usize, op1: usize, op2: usize) {
    regs[op2 as usize] = if regs[op0 as usize] == regs[op1 as usize] { 1 } else { 0 };
}

fn main() {
    let mut insns: HashMap<String, fn(&mut Vec<usize>, usize, usize, usize)> = HashMap::new();
    insns.insert(String::from("addr"), addr);
    insns.insert(String::from("addi"), addi);
    insns.insert(String::from("mulr"), mulr);
    insns.insert(String::from("muli"), muli);
    insns.insert(String::from("banr"), banr);
    insns.insert(String::from("bani"), bani);
    insns.insert(String::from("borr"), borr);
    insns.insert(String::from("bori"), bori);
    insns.insert(String::from("setr"), setr);
    insns.insert(String::from("seti"), seti);
    insns.insert(String::from("gtir"), gtir);
    insns.insert(String::from("gtri"), gtri);
    insns.insert(String::from("gtrr"), gtrr);
    insns.insert(String::from("eqir"), eqir);
    insns.insert(String::from("eqri"), eqri);
    insns.insert(String::from("eqrr"), eqrr);

    let mut file = File::open("/home/jie/projects/rust/advent/day19_1/input")
        .expect("Error cannot open the file");
    let mut contents = String::new();
    let mut program = Vec::new();
    let mut ip_reg = 0;

    file.read_to_string(&mut contents)
        .expect("Error reading input file");

    for line in contents.lines() {
        let ipip = scan_fmt!(line, "#ip {d}", usize);
        if let Some(ip) = ipip {
            ip_reg = ip;
        }
        let (opop, aa, bb, cc) = scan_fmt!(line, "{/[a-z]*/} {d} {d} {d}", String, usize, usize, usize);
        if let (Some(op), Some(a), Some(b), Some(c)) = (opop, aa, bb, cc) {
            program.push((op, a, b, c));
        }
    }
/*
    println!("IP REG is {}", ip_reg);
    for insn in program {
        println!("{}, {}, {}, {}", insn.0, insn.1, insn.2, insn.3);
    }
    println!("");
*/
    let mut regs = vec![0; 6];
    let mut ip = 0;
    loop {
//        println!("ip = {}", ip);
        regs[ip_reg] = ip;
        let f = insns.get(&program[ip].0).unwrap();
        (f)(&mut regs, program[ip].1, program[ip].2, program[ip].3);
        ip = regs[ip_reg] + 1;
        if ip >= program.len() {
            break;
        }
    }

    println!("Regs = [{}, {}, {}, {}, {}, {}]", regs[0], regs[1], regs[2], regs[3], regs[4], regs[5]);
}

