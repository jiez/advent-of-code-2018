#[macro_use] extern crate scan_fmt;

use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

struct Instruction {
    name: String,
    func: fn(regs: &mut Vec<u32>, op0: u32, op1: u32, op2: u32),
    opcode: usize,
}

struct Sample {
    before: (u32, u32, u32, u32),
    after: (u32, u32, u32, u32),
    insn: (usize, u32, u32, u32),
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
    insns.push(Instruction{name: String::from("addr"), func: addr, opcode: 0});
    insns.push(Instruction{name: String::from("addi"), func: addi, opcode: 0});
    insns.push(Instruction{name: String::from("mulr"), func: mulr, opcode: 0});
    insns.push(Instruction{name: String::from("muli"), func: muli, opcode: 0});
    insns.push(Instruction{name: String::from("banr"), func: banr, opcode: 0});
    insns.push(Instruction{name: String::from("bani"), func: bani, opcode: 0});
    insns.push(Instruction{name: String::from("borr"), func: borr, opcode: 0});
    insns.push(Instruction{name: String::from("bori"), func: bori, opcode: 0});
    insns.push(Instruction{name: String::from("setr"), func: setr, opcode: 0});
    insns.push(Instruction{name: String::from("seti"), func: seti, opcode: 0});
    insns.push(Instruction{name: String::from("gtir"), func: gtir, opcode: 0});
    insns.push(Instruction{name: String::from("gtri"), func: gtri, opcode: 0});
    insns.push(Instruction{name: String::from("gtrr"), func: gtrr, opcode: 0});
    insns.push(Instruction{name: String::from("eqir"), func: eqir, opcode: 0});
    insns.push(Instruction{name: String::from("eqri"), func: eqri, opcode: 0});
    insns.push(Instruction{name: String::from("eqrr"), func: eqrr, opcode: 0});

    let num_of_insns = insns.len();
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

        let (opop, aa, bb, cc) = scan_fmt!(line, "{d} {d} {d} {d}", usize, u32, u32, u32);
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

    let mut possible_insns = vec![HashSet::new(); num_of_insns];
    for opcode in 0..num_of_insns {
        for insn_idx in 0..num_of_insns {
            possible_insns[opcode].insert(insn_idx);
        }
    }
        
    for s in samples {
        let mut possible_insns_for_this_sample = HashSet::new();
        for (idx, i) in insns.iter().enumerate() {
            let mut regs = vec![s.before.0, s.before.1, s.before.2, s.before.3];
            (i.func)(&mut regs, s.insn.1, s.insn.2, s.insn.3);
            if (regs[0], regs[1], regs[2], regs[3]) == (s.after.0, s.after.1, s.after.2, s.after.3) {
                possible_insns_for_this_sample.insert(idx);
            }
        }
        possible_insns[s.insn.0].retain(|&x| possible_insns_for_this_sample.contains(&x));
    }

    println!("possible insns:");
    for opcode in 0..num_of_insns {
        print!("{}:", opcode);
        for insn_idx in &possible_insns[opcode] {
            print!(" {}", insn_idx);
        }
        println!("");
    }
    println!("");

    let mut count = 0;
    'outer: loop {
        for opcode in 0..num_of_insns {
            if possible_insns[opcode].len() == 1 {
                let mut insn_idx = 0;
                for ii in &possible_insns[opcode] {
                    insn_idx = *ii;
                }
                insns[insn_idx].opcode = opcode;
                println!("opcode {} is insn {}", opcode, insn_idx);
                for j in 0..num_of_insns {
                    possible_insns[j].remove(&insn_idx);
                }
                count += 1;
                if count == num_of_insns {
                    break 'outer;
                }
            }
        }
    }

    for i in &insns {
        println!("{}: {}", i.name, i.opcode);
    }

    insns.sort_by_key(|insn| insn.opcode);

    for i in &insns {
        println!("{}: {}", i.name, i.opcode);
    }

    // execute the test program
    let mut last_line_is_after = false;
    let mut regs = vec![0; 4];
    for line in contents.lines() {
        let (rr0, rr1, rr2, rr3) = scan_fmt!(line, "Before: [{d}, {d}, {d}, {d}]", u32, u32, u32, u32);
        if let (Some(_r0), Some(_r1), Some(_r2), Some(_r3)) = (rr0, rr1, rr2, rr3) {
            last_line_is_after = false;
        }

        let (rr0, rr1, rr2, rr3) = scan_fmt!(line, "After: [{d}, {d}, {d}, {d}]", u32, u32, u32, u32);
        if let (Some(_r0), Some(_r1), Some(_r2), Some(_r3)) = (rr0, rr1, rr2, rr3) {
            last_line_is_after = true;
        }

        let (opop, aa, bb, cc) = scan_fmt!(line, "{d} {d} {d} {d}", usize, u32, u32, u32);
        if let (Some(op), Some(a), Some(b), Some(c)) = (opop, aa, bb, cc) {
            if last_line_is_after {
                (insns[op].func)(&mut regs, a, b, c);
            }
        }
    }

    println!("Reg[0] = {}", regs[0]);
}

