#[macro_use] extern crate scan_fmt;

use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

struct Instruction {
    name: String,
    func: fn(regs: &mut Vec<u32>, op0: u32, op1: u32, op2: u32),
}

struct Sample {
    before: (u32, u32, u32, u32),
    after: (u32, u32, u32, u32),
    insn: (u32, u32, u32, u32),
}

fn addr(regs: &mut Vec<u32>, op0: u32, op1: u32, op2: u32) {
    regs[op2 as usize] = regs[op0 as usize] + regs[op1 as usize];
}

fn addi(regs: &mut Vec<u32>, op0: u32, op1: u32, op2: u32) {
    regs[op2 as usize] = regs[op0 as usize] + op1;
}

fn mulr(regs: &mut Vec<u32>, op0: u32, op1: u32, op2: u32) {
    regs[op2 as usize] = regs[op0 as usize] * regs[op1 as usize];
}

fn muli(regs: &mut Vec<u32>, op0: u32, op1: u32, op2: u32) {
    regs[op2 as usize] = regs[op0 as usize] * op1;
}

fn banr(regs: &mut Vec<u32>, op0: u32, op1: u32, op2: u32) {
    regs[op2 as usize] = regs[op0 as usize] & regs[op1 as usize];
}

fn bani(regs: &mut Vec<u32>, op0: u32, op1: u32, op2: u32) {
    regs[op2 as usize] = regs[op0 as usize] & op1;
}

fn borr(regs: &mut Vec<u32>, op0: u32, op1: u32, op2: u32) {
    regs[op2 as usize] = regs[op0 as usize] | regs[op1 as usize];
}

fn bori(regs: &mut Vec<u32>, op0: u32, op1: u32, op2: u32) {
    regs[op2 as usize] = regs[op0 as usize] | op1;
}

fn setr(regs: &mut Vec<u32>, op0: u32, op1: u32, op2: u32) {
    regs[op2 as usize] = regs[op0 as usize];
}

fn seti(regs: &mut Vec<u32>, op0: u32, op1: u32, op2: u32) {
    regs[op2 as usize] = op0;
}

fn gtir(regs: &mut Vec<u32>, op0: u32, op1: u32, op2: u32) {
    regs[op2 as usize] = if op0 > regs[op1 as usize] { 1 } else { 0 };
}

fn gtri(regs: &mut Vec<u32>, op0: u32, op1: u32, op2: u32) {
    regs[op2 as usize] = if regs[op0 as usize] > op1 { 1 } else { 0 };
}

fn gtrr(regs: &mut Vec<u32>, op0: u32, op1: u32, op2: u32) {
    regs[op2 as usize] = if regs[op0 as usize] > regs[op1 as usize] { 1 } else { 0 };
}

fn eqir(regs: &mut Vec<u32>, op0: u32, op1: u32, op2: u32) {
    regs[op2 as usize] = if op0 == regs[op1 as usize] { 1 } else { 0 };
}

fn eqri(regs: &mut Vec<u32>, op0: u32, op1: u32, op2: u32) {
    regs[op2 as usize] = if regs[op0 as usize] == op1 { 1 } else { 0 };
}

fn eqrr(regs: &mut Vec<u32>, op0: u32, op1: u32, op2: u32) {
    regs[op2 as usize] = if regs[op0 as usize] == regs[op1 as usize] { 1 } else { 0 };
}

fn main() {
    let mut insns = Vec::new();
    insns.push(Instruction{name: String::from("addr"), func: addr});
    insns.push(Instruction{name: String::from("addi"), func: addi});
    insns.push(Instruction{name: String::from("mulr"), func: mulr});
    insns.push(Instruction{name: String::from("muli"), func: muli});
    insns.push(Instruction{name: String::from("banr"), func: banr});
    insns.push(Instruction{name: String::from("bani"), func: bani});
    insns.push(Instruction{name: String::from("borr"), func: borr});
    insns.push(Instruction{name: String::from("bori"), func: bori});
    insns.push(Instruction{name: String::from("setr"), func: setr});
    insns.push(Instruction{name: String::from("seti"), func: seti});
    insns.push(Instruction{name: String::from("gtir"), func: gtir});
    insns.push(Instruction{name: String::from("gtri"), func: gtri});
    insns.push(Instruction{name: String::from("gtrr"), func: gtrr});
    insns.push(Instruction{name: String::from("eqir"), func: eqir});
    insns.push(Instruction{name: String::from("eqri"), func: eqri});
    insns.push(Instruction{name: String::from("eqrr"), func: eqrr});

    let mut file = File::open("/home/jie/projects/rust/advent/day16_1/input")
        .expect("Error cannot open the file");
    let mut contents = String::new();
    let mut samples = Vec::new();

    file.read_to_string(&mut contents)
        .expect("Error reading input file");

    let mut before = (0, 0, 0, 0);
    let mut after = (0, 0, 0, 0);
    let mut insn = (0, 0, 0, 0);
    for line in contents.lines() {
        let (rr0, rr1, rr2, rr3) = scan_fmt!(line, "Before: [{d}, {d}, {d}, {d}]", u32, u32, u32, u32);
        if let (Some(r0), Some(r1), Some(r2), Some(r3)) = (rr0, rr1, rr2, rr3) {
            before = (r0, r1, r2, r3);
        }

        let (rr0, rr1, rr2, rr3) = scan_fmt!(line, "After: [{d}, {d}, {d}, {d}]", u32, u32, u32, u32);
        if let (Some(r0), Some(r1), Some(r2), Some(r3)) = (rr0, rr1, rr2, rr3) {
            after = (r0, r1, r2, r3);
            samples.push(Sample{before: before, insn: insn, after: after});
        }

        let (opop, aa, bb, cc) = scan_fmt!(line, "{d} {d} {d} {d}", u32, u32, u32, u32);
        if let (Some(op), Some(a), Some(b), Some(c)) = (opop, aa, bb, cc) {
            insn = (op, a, b, c);
        }
    }

/*
    for s in samples {
        println!("Before: [{}, {}, {}, {}]", s.before.0, s.before.1, s.before.2, s.before.3);
        println!("{} {} {} {}", s.insn.0, s.insn.1, s.insn.2, s.insn.3);
        println!("After: [{}, {}, {}, {}]", s.after.0, s.after.1, s.after.2, s.after.3);
        println!("");
    }
*/
    let mut num_of_samples = 0;
    for s in samples {
        let mut count = 0;
        for i in &insns {
            let mut regs = vec![s.before.0, s.before.1, s.before.2, s.before.3];
            (i.func)(&mut regs, s.insn.1, s.insn.2, s.insn.3);
            if (regs[0], regs[1], regs[2], regs[3]) == (s.after.0, s.after.1, s.after.2, s.after.3) {
                count += 1;
            }
        }
        if count >= 3 {
            num_of_samples += 1;
        }
    }
            
    println!("num of samples behave like 3 or more opcodes: {}", num_of_samples);
}

